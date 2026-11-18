import UIKit
import UniformTypeIdentifiers

@main
final class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?
    private var library: LibraryController?
    func application(_ application: UIApplication, didFinishLaunchingWithOptions options: [UIApplication.LaunchOptionsKey: Any]? = nil) -> Bool {
        let library = LibraryController()
        self.library = library
        let window = UIWindow(frame: UIScreen.main.bounds)
        window.rootViewController = UINavigationController(rootViewController: library)
        window.makeKeyAndVisible()
        self.window = window
        if let url = options?[.url] as? URL { _ = library.importURL(url) }
        return true
    }
    func application(_ app: UIApplication, open url: URL, options: [UIApplication.OpenURLOptionsKey: Any] = [:]) -> Bool {
        _ = library?.importURL(url)
        return true
    }
}

final class LibraryController: UITableViewController, UIDocumentPickerDelegate {
    private let fm = FileManager.default
    private var files: [URL] = []
    private var info: [String: [String: String]] = [:]
    private var pending = Set<String>()
    private var apps: URL { fm.urls(for: .documentDirectory, in: .userDomainMask)[0].appendingPathComponent("apps", isDirectory: true) }
    private var cacheURL: URL { apps.appendingPathComponent(".metadata.json") }

    override func viewDidLoad() {
        super.viewDidLoad()
        title = "Library"
        navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: .add, target: self, action: #selector(pick))
        try? fm.createDirectory(at: apps, withIntermediateDirectories: true)
        if let data = try? Data(contentsOf: cacheURL), let cache = try? JSONSerialization.jsonObject(with: data) as? [String: [String: String]] { info = cache }
        tableView.register(UITableViewCell.self, forCellReuseIdentifier: "app")
    }
    override func viewWillAppear(_ animated: Bool) { super.viewWillAppear(animated); refresh() }
    @objc private func pick() {
        let picker = UIDocumentPickerViewController(forOpeningContentTypes: [UTType(filenameExtension: "apk") ?? .data], asCopy: true)
        picker.delegate = self
        picker.allowsMultipleSelection = true
        present(picker, animated: true)
    }
    func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) { urls.forEach { _ = importURL($0) } }
    func importURL(_ url: URL, refreshAfter: Bool = true) -> Bool {
        guard url.pathExtension.lowercased() == "apk" else { showError("Please choose an APK file."); return false }
        let accessing = url.startAccessingSecurityScopedResource()
        defer { if accessing { url.stopAccessingSecurityScopedResource() } }
        do {
            try fm.createDirectory(at: apps, withIntermediateDirectories: true)
            var destination = apps.appendingPathComponent(url.lastPathComponent)
            if destination.standardizedFileURL == url.standardizedFileURL { if refreshAfter { refresh() }; return true }
            let base = url.deletingPathExtension().lastPathComponent
            var n = 2
            while fm.fileExists(atPath: destination.path) {
                destination = apps.appendingPathComponent("\(base) (\(n)).apk")
                n += 1
            }
            try fm.copyItem(at: url, to: destination)
            if refreshAfter { refresh() }
            return true
        } catch { showError(error.localizedDescription); return false }
    }
    private func refresh() {
        try? fm.createDirectory(at: apps, withIntermediateDirectories: true)
        // Files/Finder exposes Documents itself. Move dropped APKs into the library.
        let documents = apps.deletingLastPathComponent()
        for url in (try? fm.contentsOfDirectory(at: documents, includingPropertiesForKeys: nil)) ?? [] where url.pathExtension.lowercased() == "apk" {
            if importURL(url, refreshAfter: false) { try? fm.removeItem(at: url) }
        }
        files = ((try? fm.contentsOfDirectory(at: apps, includingPropertiesForKeys: nil)) ?? []).filter { $0.pathExtension.lowercased() == "apk" }.sorted { $0.lastPathComponent.localizedStandardCompare($1.lastPathComponent) == .orderedAscending }
        tableView.reloadData()
        for url in files where info[url.lastPathComponent] == nil && !pending.contains(url.lastPathComponent) {
            let name = url.lastPathComponent
            pending.insert(name)
            let worker = VMWorker()
            worker.open(url) { [weak self, worker] json, _ in
                defer { worker.close() }
                guard let self = self else { return }
                self.pending.remove(name)
                if let json = json, let data = json.data(using: .utf8), let parsed = try? JSONSerialization.jsonObject(with: data) as? [String: String], self.fm.fileExists(atPath: url.path) {
                    self.info[name] = parsed
                    self.saveCache()
                    self.tableView.reloadData()
                }
            }
        }
    }
    private func saveCache() {
        if let data = try? JSONSerialization.data(withJSONObject: info) { try? data.write(to: cacheURL, options: .atomic) }
    }
    private func showError(_ message: String) {
        let alert = UIAlertController(title: "Import failed", message: message, preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "OK", style: .default))
        present(alert, animated: true)
    }
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int { files.count }
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "app", for: indexPath)
        let file = files[indexPath.row]
        var content = cell.defaultContentConfiguration()
        content.text = info[file.lastPathComponent]?["label"].flatMap { $0.isEmpty ? nil : $0 } ?? file.deletingPathExtension().lastPathComponent
        content.secondaryText = info[file.lastPathComponent]?["package"] ?? file.lastPathComponent
        cell.contentConfiguration = content
        cell.accessoryType = .disclosureIndicator
        return cell
    }
    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
        navigationController?.pushViewController(RunnerController(url: files[indexPath.row]), animated: true)
    }
    override func tableView(_ tableView: UITableView, commit editingStyle: UITableViewCell.EditingStyle, forRowAt indexPath: IndexPath) {
        guard editingStyle == .delete else { return }
        let file = files[indexPath.row]
        do { try fm.removeItem(at: file); info.removeValue(forKey: file.lastPathComponent); saveCache(); refresh() }
        catch { showError(error.localizedDescription) }
    }
}

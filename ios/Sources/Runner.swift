import UIKit
import UIKit.UIGestureRecognizerSubclass

final class RunnerController: UIViewController, UINavigationControllerDelegate {
    private let url: URL
    private let worker = VMWorker()
    private let banner = UILabel()
    private let content = UIView()
    private var windowTouch: WindowTouch?
    private var nodes: [String: NodeView] = [:]
    private var usedNodes = Set<String>()
    private var lastWindowSize: CGSize?
    private var closing = false
    private var opened = false
    private var previousDelegate: UINavigationControllerDelegate?
    private var tickTimer: Timer?
    private var tickInFlight = false
    private var touchInFlight = false
    private var touchQueue: [(action: Int32, point: CGPoint, longID: Int32?)] = []
    private var activeTouches = 0
    private var presentedDialog: String?
    private var dialogHost: UIViewController?
    private let toastLabel = UILabel()
    private var imageCache: [Int32: UIImage] = [:]
    private var missingImages = Set<Int32>()
    private var pendingImages = Set<Int32>()
    private var lastUI: [String: Any]?

    init(url: URL) { self.url = url; super.init(nibName: nil, bundle: nil) }
    required init?(coder: NSCoder) { fatalError("init(coder:) is unavailable") }
    override func viewDidLoad() {
        super.viewDidLoad()
        title = url.deletingPathExtension().lastPathComponent
        view.backgroundColor = .systemBackground
        navigationItem.hidesBackButton = true
        navigationItem.leftBarButtonItem = UIBarButtonItem(title: "Back", style: .plain, target: self, action: #selector(back))
        navigationItem.rightBarButtonItem = UIBarButtonItem(title: "Log", style: .plain, target: self, action: #selector(showLog))
        content.translatesAutoresizingMaskIntoConstraints = false
        banner.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(content)
        let touch = WindowTouch { [weak self] action, point in self?.forwardTouch(action, point) }
        touch.isEnabled = false
        content.addGestureRecognizer(touch)
        windowTouch = touch
        view.addSubview(banner)
        NSLayoutConstraint.activate([content.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor), content.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor), content.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor), content.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor), banner.topAnchor.constraint(equalTo: content.topAnchor), banner.leadingAnchor.constraint(equalTo: content.leadingAnchor), banner.trailingAnchor.constraint(equalTo: content.trailingAnchor)])
        banner.textColor = .white
        banner.backgroundColor = .systemRed
        banner.numberOfLines = 0
        banner.isHidden = true
        toastLabel.textColor = .white
        toastLabel.backgroundColor = UIColor.black.withAlphaComponent(0.8)
        toastLabel.textAlignment = .center
        toastLabel.numberOfLines = 0
        toastLabel.layer.cornerRadius = 12
        toastLabel.clipsToBounds = true
        toastLabel.alpha = 0
        toastLabel.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(toastLabel)
        NSLayoutConstraint.activate([toastLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor), toastLabel.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -48), toastLabel.widthAnchor.constraint(lessThanOrEqualTo: view.widthAnchor, multiplier: 0.85)])
        worker.open(url) { [weak self] _, error in
            guard let self = self, !self.closing else { return }
            if let error = error { self.showError(error); return }
            self.opened = true
            self.windowTouch?.isEnabled = true
            let data = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0].appendingPathComponent("data", isDirectory: true).path
            self.send { vm in
                if let r = data.withCString({ idorid_vm_set_data_dir(vm, $0) }) { idorid_string_free(r) }
                return idorid_vm_launch(vm)
            }
            self.reportWindowSize()
        }
    }
    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        reportWindowSize()
    }
    private func reportWindowSize() {
        guard opened else { return }
        let w = content.bounds.width.rounded(), h = content.bounds.height.rounded()
        guard w > 0, h > 0, w <= CGFloat(Int32.max), h <= CGFloat(Int32.max) else { return }
        let size = CGSize(width: w, height: h)
        guard size != lastWindowSize else { return }
        lastWindowSize = size
        send { idorid_vm_set_window_size($0, Int32(w), Int32(h)) }
    }
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        if let nav = navigationController {
            previousDelegate = nav.delegate
            nav.delegate = self
            nav.interactivePopGestureRecognizer?.isEnabled = false
        }
    }
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        if isMovingFromParent {
            closing = true
            tickTimer?.invalidate()
            worker.close()
            navigationController?.delegate = previousDelegate
            navigationController?.interactivePopGestureRecognizer?.isEnabled = true
        }
    }
    private func send(completion: (() -> Void)? = nil, _ action: @escaping (OpaquePointer) -> UnsafeMutablePointer<CChar>?) {
        worker.perform(action) { [weak self] json in
            completion?()
            guard let self = self, !self.closing else { return }
            guard let json = json, let data = json.data(using: .utf8), let object = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
                self.showError("The VM returned invalid UI JSON."); return
            }
            self.render(object)
        }
    }
    @objc private func back() {
        if !opened { closing = true; worker.close(); navigationController?.popViewController(animated: true); return }
        send { idorid_vm_back($0) }
    }
    @objc private func showLog() {
        worker.logs { [weak self] log in
            guard let self = self, !self.closing else { return }
            let controller = UIViewController()
            controller.title = "Log"
            controller.view.backgroundColor = .systemBackground
            let text = UITextView(frame: .zero)
            text.isEditable = false
            text.font = .monospacedSystemFont(ofSize: 12, weight: .regular)
            text.text = log
            text.translatesAutoresizingMaskIntoConstraints = false
            controller.view.addSubview(text)
            NSLayoutConstraint.activate([text.leadingAnchor.constraint(equalTo: controller.view.leadingAnchor), text.trailingAnchor.constraint(equalTo: controller.view.trailingAnchor), text.topAnchor.constraint(equalTo: controller.view.safeAreaLayoutGuide.topAnchor), text.bottomAnchor.constraint(equalTo: controller.view.bottomAnchor)])
            controller.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: .done, target: self, action: #selector(self.dismissLog))
            self.present(UINavigationController(rootViewController: controller), animated: true)
        }
    }
    @objc private func dismissLog() { dismiss(animated: true) }
    private func showError(_ message: String?) { banner.text = message; banner.isHidden = message == nil || message?.isEmpty == true }
    private func scheduleTick(_ next: Any?) {
        tickTimer?.invalidate()
        guard let ms = (next as? NSNumber)?.doubleValue, ms.isFinite, ms >= 0 else { return }
        let timer = Timer(timeInterval: max(0.001, ms / 1000), repeats: false) { [weak self] _ in
            guard let self = self, !self.closing else { return }
            if self.tickInFlight { self.scheduleTick(NSNumber(value: 16)); return }
            self.tickInFlight = true
            self.send(completion: { [weak self] in self?.tickInFlight = false }) { idorid_vm_tick($0) }
        }
        tickTimer = timer
        RunLoop.main.add(timer, forMode: .common)
    }
    private func forwardTouch(_ action: Int32, _ point: CGPoint) {
        guard opened, !closing else { return }
        guard point.x.isFinite, point.y.isFinite else { return }
        if action == 0 { activeTouches += 1 }
        if action == 2, touchQueue.last?.action == 2 {
            touchQueue[touchQueue.count - 1] = (action, point, nil)
        } else {
            touchQueue.append((action, point, nil))
        }
        deliverNextTouch()
    }
    private func forwardLongClick(_ id: Int32) {
        guard opened, !closing else { return }
        windowTouch?.cancelCurrentTouch()
        touchQueue.append((4, .zero, id))
        deliverNextTouch()
    }
    private func deliverNextTouch() {
        guard !touchInFlight, !touchQueue.isEmpty, !closing else { return }
        let event = touchQueue.removeFirst()
        touchInFlight = true
        let x = Float(event.point.x), y = Float(event.point.y)
        send(completion: { [weak self] in
            guard let self = self else { return }
            self.touchInFlight = false
            if event.action == 1 || event.action == 3 { self.activeTouches = max(0, self.activeTouches - 1) }
            self.deliverNextTouch()
        }) { vm in
            if let id = event.longID { return idorid_vm_long_click(vm, id) }
            return idorid_vm_touch_window(vm, event.action, x, y)
        }
    }
    private func reportScroll(_ id: Int32, _ offset: CGPoint) {
        let x = offset.x.rounded(), y = offset.y.rounded()
        guard x.isFinite, y.isFinite, x >= CGFloat(Int32.min), x <= CGFloat(Int32.max), y >= CGFloat(Int32.min), y <= CGFloat(Int32.max), opened, !closing else { return }
        worker.perform({ idorid_vm_scroll($0, id, Int32(x), Int32(y)) }) { _ in }
    }
    private func showToasts(_ toasts: [String]) {
        guard let text = toasts.last else { return }
        toastLabel.text = "  \(text)  "
        UIView.animate(withDuration: 0.2) { self.toastLabel.alpha = 1 }
        UIView.animate(withDuration: 0.3, delay: 2.0, options: []) { self.toastLabel.alpha = 0 }
    }
    private func showDialog(_ dialog: [String: Any]?) {
        guard let dialog = dialog else {
            if presentedDialog != nil, presentedViewController is UIAlertController { dismiss(animated: true) }
            presentedDialog = nil
            dialogHost = nil
            return
        }
        var chrome = dialog
        chrome.removeValue(forKey: "content")
        let key = (try? JSONSerialization.data(withJSONObject: chrome, options: [.sortedKeys])).flatMap { String(data: $0, encoding: .utf8) } ?? "dialog"
        if key == presentedDialog { return }
        if presentedViewController is UIAlertController { dismiss(animated: false) }
        presentedDialog = key
        let items = dialog["items"] as? [String] ?? []
        let alert = UIAlertController(title: dialog["title"] as? String, message: dialog["message"] as? String, preferredStyle: items.isEmpty ? .alert : .actionSheet)
        dialogHost = nil
        if let node = dialog["content"] as? [String: Any] {
            let host = UIViewController()
            let size = nodeFrame(node).size
            host.preferredContentSize = size
            host.view = UIView(frame: CGRect(origin: .zero, size: size))
            alert.setValue(host, forKey: "contentViewController")
            dialogHost = host
        }
        for (index, item) in items.enumerated() {
            let which = Int32(-100 - index)
            alert.addAction(UIAlertAction(title: item, style: .default) { [weak self] _ in self?.presentedDialog = nil; self?.send { idorid_vm_click($0, which) } })
        }
        for button in dialog["buttons"] as? [[String: Any]] ?? [] {
            let which = (button["which"] as? NSNumber)?.int32Value ?? -1
            let style: UIAlertAction.Style = which == -2 ? .cancel : .default
            alert.addAction(UIAlertAction(title: button["text"] as? String, style: style) { [weak self] _ in self?.presentedDialog = nil; self?.send { idorid_vm_click($0, which) } })
        }
        if alert.actions.isEmpty {
            alert.addAction(UIAlertAction(title: "OK", style: .cancel) { [weak self] _ in self?.presentedDialog = nil; self?.send { idorid_vm_back($0) } })
        }
        if let pop = alert.popoverPresentationController { pop.sourceView = view; pop.sourceRect = CGRect(x: view.bounds.midX, y: view.bounds.midY, width: 1, height: 1) }
        present(alert, animated: true)
    }
    private func requestImage(_ id: Int32) {
        guard !pendingImages.contains(id) else { return }
        pendingImages.insert(id)
        worker.images([id]) { [weak self] images in
            guard let self = self, !self.closing else { return }
            self.pendingImages.remove(id)
            if let image = images[id] { self.imageCache[id] = image } else { self.missingImages.insert(id) }
            if images[id] != nil {
                self.nodes.values.forEach { $0.canvas?.setNeedsDisplay() }
                if self.activeTouches == 0, let ui = self.lastUI { self.render(ui) }
            }
        }
    }
    private func render(_ object: [String: Any]) {
        lastUI = object
        if object["finished"] as? Bool == true { closing = true; tickTimer?.invalidate(); worker.close(); navigationController?.popViewController(animated: true); return }
        scheduleTick(object["next"])
        showToasts(object["toasts"] as? [String] ?? [])
        showDialog(object["dialog"] as? [String: Any])
        if let title = object["title"] as? String { self.title = title }
        showError(object["error"] as? String)
        usedNodes.removeAll()
        if let root = object["root"] as? [String: Any] {
            let child = build(root, parentKey: "root")
            if child.superview !== content { content.addSubview(child) }
        }
        if let dialog = object["dialog"] as? [String: Any], let root = dialog["content"] as? [String: Any], let host = dialogHost {
            let child = build(root, parentKey: "dialog", inDialog: true)
            if child.superview !== host.view { host.view.addSubview(child) }
            host.preferredContentSize = child.frame.size
            host.view.frame = CGRect(origin: .zero, size: child.frame.size)
        }
        for (key, node) in nodes where !usedNodes.contains(key) { node.removeFromSuperview(); nodes.removeValue(forKey: key) }
    }

}

/// Tap recognizer that calls a closure.
final class ClosureTap: UITapGestureRecognizer {
    private let action: () -> Void
    init(_ action: @escaping () -> Void) {
        self.action = action
        super.init(target: nil, action: nil)
        addTarget(self, action: #selector(fire))
    }
    @objc private func fire() { action() }
}

/// Tracks one finger and leaves child controls free to handle their own gestures.
final class WindowTouch: UIGestureRecognizer, UIGestureRecognizerDelegate {
    private let forward: (Int32, CGPoint) -> Void
    private var tracked: UITouch?
    private var forwarded = false
    init(_ forward: @escaping (Int32, CGPoint) -> Void) {
        self.forward = forward
        super.init(target: nil, action: nil)
        cancelsTouchesInView = false
        delaysTouchesBegan = false
        delaysTouchesEnded = false
        delegate = self
    }
    func gestureRecognizer(_ gestureRecognizer: UIGestureRecognizer, shouldRecognizeSimultaneouslyWith otherGestureRecognizer: UIGestureRecognizer) -> Bool { true }
    func gestureRecognizer(_ gestureRecognizer: UIGestureRecognizer, shouldRequireFailureOf otherGestureRecognizer: UIGestureRecognizer) -> Bool { false }
    func gestureRecognizer(_ gestureRecognizer: UIGestureRecognizer, shouldBeRequiredToFailBy otherGestureRecognizer: UIGestureRecognizer) -> Bool { false }
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent) {
        guard tracked == nil, let touch = touches.first, let view = view else { return }
        tracked = touch; forwarded = true; state = .began; forward(0, touch.location(in: view))
    }
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent) {
        guard let touch = tracked, touches.contains(touch), let view = view else { return }
        state = .changed; if forwarded { forward(2, touch.location(in: view)) }
    }
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent) {
        guard let touch = tracked, touches.contains(touch), let view = view else { return }
        if forwarded { forward(1, touch.location(in: view)) }; forwarded = false; tracked = nil; state = .ended
    }
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent) {
        guard let touch = tracked, touches.contains(touch), let view = view else { return }
        if forwarded { forward(3, touch.location(in: view)) }; forwarded = false; tracked = nil; state = .cancelled
    }
    func cancelCurrentTouch() {
        guard forwarded, let touch = tracked, let view = view else { return }
        forwarded = false
        forward(3, touch.location(in: view))
    }
    override func reset() {
        cancelCurrentTouch()
        tracked = nil
        super.reset()
    }
}

private func androidFont(size: CGFloat, bold: Bool, italic: Bool, monospace: Bool) -> UIFont {
    let weight: UIFont.Weight = bold ? .bold : .regular
    var font = monospace ? UIFont.monospacedSystemFont(ofSize: size, weight: weight) : UIFont.systemFont(ofSize: size, weight: weight)
    if italic, let descriptor = font.fontDescriptor.withSymbolicTraits(font.fontDescriptor.symbolicTraits.union(.traitItalic)) {
        font = UIFont(descriptor: descriptor, size: size)
    }
    return font
}

let idoridTextMeasurer: @convention(c) (UnsafePointer<CChar>?, Float, Int32, Float, UnsafeMutablePointer<Float>?) -> Void = { text, size, flags, maxWidth, out in
    guard let out = out else { return }
    let font = androidFont(size: CGFloat(size), bold: flags & 1 != 0, italic: flags & 2 != 0, monospace: flags & 4 != 0)
    let string = text.map { String(cString: $0) } ?? ""
    let width = maxWidth > 0 ? CGFloat(maxWidth) : CGFloat.greatestFiniteMagnitude
    let rect = (string as NSString).boundingRect(with: CGSize(width: width, height: .greatestFiniteMagnitude), options: [.usesLineFragmentOrigin, .usesFontLeading], attributes: [.font: font], context: nil)
    let lines = Int((flags >> 8) & 255)
    let height = lines > 0 ? min(rect.height, CGFloat(lines) * font.lineHeight) : rect.height
    out[0] = Float(ceil(rect.width)); out[1] = Float(ceil(height))
}

private func nodeFrame(_ node: [String: Any]) -> CGRect {
    guard let a = node["frame"] as? [NSNumber], a.count == 4 else { return .zero }
    let v = a.map { CGFloat($0.doubleValue) }
    guard v.allSatisfy({ $0.isFinite }) else { return .zero }
    return CGRect(x: v[0], y: v[1], width: max(0, v[2]), height: max(0, v[3]))
}

private func nodeInsets(_ node: [String: Any]) -> UIEdgeInsets {
    guard let a = node["padding"] as? [NSNumber], a.count == 4 else { return .zero }
    return UIEdgeInsets(top: CGFloat(a[1].doubleValue), left: CGFloat(a[0].doubleValue), bottom: CGFloat(a[3].doubleValue), right: CGFloat(a[2].doubleValue))
}

private func nodeColor(_ value: Any?) -> UIColor? {
    guard let s = value as? String, s.count == 9, s.first == "#", let n = UInt32(s.dropFirst(), radix: 16) else { return nil }
    return UIColor(red: CGFloat((n >> 16) & 255) / 255, green: CGFloat((n >> 8) & 255) / 255, blue: CGFloat(n & 255) / 255, alpha: CGFloat((n >> 24) & 255) / 255)
}

private final class NodeView: UIView, UITextViewDelegate, UIScrollViewDelegate {
    let kind: String
    let id: Int32
    var textChanged: ((String) -> Void)?
    var editorAction: (() -> Void)?
    var progressChanged: ((Int32) -> Void)?
    var scrollChanged: ((CGPoint) -> Void)?
    var scrollBegan: (() -> Void)?
    var canvas: CanvasView?
    var widget: UIView?
    var label: UILabel?
    var scroll: UIScrollView?
    var padding = UIEdgeInsets.zero
    var gravity = ""
    var isUpdating = false
    var gestureMode = ""
    var placeholder: UILabel?

    /** True when canvas ops contain an actual drawing (not just save/restore/transforms). */
    static func hasVisibleDrawing(_ ops: Any?) -> Bool {
        guard let list = ops as? [Any] else { return false }
        for op in list {
            if let a = op as? [Any], let name = a.first as? String, !["save", "restore", "translate", "scale", "rotate", "concat", "clip"].contains(name) { return true }
        }
        return false
    }

    init(kind: String, id: Int32) {
        self.kind = kind; self.id = id
        super.init(frame: .zero)
        switch kind {
        case "ScrollView", "HorizontalScrollView":
            let v = UIScrollView(); v.delegate = self; addSubview(v); scroll = v
        case "TextView":
            let v = UILabel(); v.numberOfLines = 0; v.lineBreakMode = .byWordWrapping; addSubview(v); label = v
        case "Button", "ImageButton", "CheckBox", "RadioButton", "Spinner":
            let v = UIButton(type: .system); addSubview(v); widget = v
            v.adjustsImageWhenHighlighted = false
        case "EditText":
            let v = UITextField(); v.borderStyle = .none; addSubview(v); widget = v
            v.addTarget(self, action: #selector(fieldChanged), for: .editingChanged)
            v.addTarget(self, action: #selector(fieldReturn), for: .editingDidEndOnExit)
        case "EditTextMultiline":
            let v = UITextView(); v.delegate = self; v.backgroundColor = .clear; v.textContainerInset = .zero; v.textContainer.lineFragmentPadding = 0; addSubview(v); widget = v
            let hint = UILabel(); hint.textColor = .placeholderText; hint.numberOfLines = 0; hint.isUserInteractionEnabled = false; addSubview(hint); placeholder = hint
        case "Switch":
            let v = UISwitch(); addSubview(v); widget = v
            let l = UILabel(); l.numberOfLines = 0; l.lineBreakMode = .byWordWrapping; addSubview(l); label = l
        case "ImageView": let v = UIImageView(); addSubview(v); widget = v
        case "ProgressBar": let v = UIProgressView(progressViewStyle: .default); addSubview(v); widget = v
        case "ProgressBarIndeterminate": let v = UIActivityIndicatorView(style: .medium); v.startAnimating(); addSubview(v); widget = v
        case "SeekBar":
            let v = UISlider(); v.addTarget(self, action: #selector(sliderChanged), for: [.touchUpInside, .touchUpOutside]); addSubview(v); widget = v
        default: break
        }
    }
    required init?(coder: NSCoder) { fatalError("init(coder:) is unavailable") }
    func scrollViewDidScroll(_ scrollView: UIScrollView) { scrollChanged?(scrollView.contentOffset) }
    func scrollViewWillBeginDragging(_ scrollView: UIScrollView) { scrollBegan?() }
    @objc private func fieldChanged() { textChanged?((widget as? UITextField)?.text ?? "") }
    @objc private func fieldReturn() { editorAction?() }
    @objc private func sliderChanged() { if let v = widget as? UISlider { progressChanged?(Int32(v.value)) } }
    func textViewDidChange(_ textView: UITextView) { placeholder?.isHidden = !(textView.text ?? "").isEmpty; if !isUpdating { textChanged?(textView.text ?? "") } }

    override func layoutSubviews() {
        super.layoutSubviews()
        canvas?.frame = bounds
        scroll?.frame = bounds
        let r = bounds.inset(by: padding)
        let area = CGRect(x: r.minX, y: r.minY, width: max(0, r.width), height: max(0, r.height))
        if let label = label, kind == "TextView" {
            let size = label.sizeThatFits(CGSize(width: area.width, height: .greatestFiniteMagnitude))
            let h = min(area.height, ceil(size.height))
            let parts = gravity.split(separator: "|").map(String.init)
            let y = parts.contains("bottom") ? area.maxY - h : (parts.contains("center") || parts.contains("center_vertical") ? area.midY - h / 2 : area.minY)
            label.frame = CGRect(x: area.minX, y: y, width: area.width, height: h)
        } else if let label = label, let toggle = widget as? UISwitch {
            let size = toggle.sizeThatFits(area.size)
            toggle.frame = CGRect(x: area.maxX - size.width, y: area.midY - size.height / 2, width: size.width, height: size.height)
            label.frame = CGRect(x: area.minX, y: area.minY, width: max(0, area.width - size.width - 8), height: area.height)
        } else { widget?.frame = area; placeholder?.frame = area }
    }
}

extension RunnerController {
    private func build(_ node: [String: Any], parentKey: String, inDialog: Bool = false) -> NodeView {
        let id = (node["id"] as? NSNumber)?.int32Value ?? 0
        let key = id == 0 ? "\(parentKey)/0" : "\(id)"
        let type = node["type"] as? String ?? "View"
        let input = (node["inputType"] as? NSNumber)?.intValue ?? 1
        var kind = type == "EditText" && (node["maxLines"] as? NSNumber)?.intValue != 1 && input & 0x20000 != 0 ? "EditTextMultiline" : (type == "ProgressBar" && (node["indeterminate"] as? Bool ?? true) ? "ProgressBarIndeterminate" : type)
        // A checkable Button subclass (SwitchCompat/MaterialSwitch) that draws nothing visible: show a native switch.
        if type == "Button" && node["checked"] != nil && !NodeView.hasVisibleDrawing(node["draw"]) { kind = "Switch" }
        let view: NodeView
        if let old = nodes[key], old.kind == kind { view = old }
        else { nodes[key]?.removeFromSuperview(); view = NodeView(kind: kind, id: id); nodes[key] = view }
        usedNodes.insert(key)
        view.frame = nodeFrame(node)
        view.padding = nodeInsets(node)
        view.gravity = node["gravity"] as? String ?? ""
        view.backgroundColor = nodeColor(node["bg"]) ?? .clear
        view.layer.cornerRadius = (node["radius"] as? NSNumber).map { CGFloat($0.doubleValue) } ?? 0
        view.clipsToBounds = view.layer.cornerRadius > 0
        view.alpha = (node["alpha"] as? NSNumber).map { CGFloat($0.doubleValue) } ?? 1
        view.isHidden = (node["visible"] as? Bool) == false || (node["visible"] as? String) == "invisible"
        let enabled = node["enabled"] as? Bool ?? true
        view.isUserInteractionEnabled = true
        (view.widget as? UIControl)?.isEnabled = enabled
        view.widget?.isUserInteractionEnabled = enabled && ["EditText", "EditTextMultiline", "Spinner", "SeekBar"].contains(kind)
        view.scrollChanged = { [weak self] offset in self?.reportScroll(id, offset) }
        view.scrollBegan = { [weak self] in self?.windowTouch?.cancelCurrentTouch() }
        view.editorAction = { [weak self] in self?.send { idorid_vm_editor_action($0, id) } }
        view.textChanged = { [weak self] text in self?.send { vm in text.withCString { idorid_vm_set_text(vm, id, $0) } } }
        view.progressChanged = { [weak self] value in self?.send { idorid_vm_set_progress($0, id, value) } }
        let font = androidFont(size: CGFloat((node["textSize"] as? NSNumber)?.doubleValue ?? 14), bold: node["bold"] as? Bool ?? false, italic: node["italic"] as? Bool ?? false, monospace: node["monospace"] as? Bool ?? false)
        let color = nodeColor(node["color"]) ?? UIColor(red: 0, green: 0, blue: 0, alpha: 222.0 / 255.0)
        if let label = view.label {
            label.text = node["text"] as? String; label.font = font; label.textColor = color
            label.numberOfLines = (node["maxLines"] as? NSNumber)?.intValue ?? 0
            let parts = view.gravity.split(separator: "|").map(String.init)
            label.textAlignment = parts.contains("right") ? .right : (parts.contains("center") || parts.contains("center_horizontal") ? .center : .left)
        }
        if let button = view.widget as? UIButton {
            button.titleLabel?.font = font; button.setTitleColor(color, for: .normal)
            button.contentHorizontalAlignment = .center
            switch type {
            case "CheckBox", "RadioButton":
                let checked = node["checked"] as? Bool ?? false
                if node["customButton"] as? Bool == true && NodeView.hasVisibleDrawing(node["draw"]) {
                    // The app draws its own check mark / chip: only place the text after it.
                    let inset = CGFloat((node["textInset"] as? NSNumber)?.doubleValue ?? 0)
                    button.setImage(nil, for: .normal); button.setTitle(node["text"] as? String, for: .normal)
                    button.contentHorizontalAlignment = inset > 0 ? .leading : .center
                    button.contentEdgeInsets = UIEdgeInsets(top: 0, left: inset, bottom: 0, right: 0)
                } else {
                    let symbol = type == "CheckBox" ? (checked ? "checkmark.square.fill" : "square") : (checked ? "largecircle.fill.circle" : "circle")
                    button.setImage(UIImage(systemName: symbol), for: .normal); button.setTitle("  " + (node["text"] as? String ?? ""), for: .normal); button.contentHorizontalAlignment = .leading
                    button.contentEdgeInsets = .zero
                }
            case "ImageButton": button.setImage(UIImage(systemName: "photo"), for: .normal)
            case "Spinner":
                let items = node["items"] as? [[String: Any]] ?? []; let selected = (node["selected"] as? NSNumber)?.intValue ?? -1
                button.setTitle((items.indices.contains(selected) ? (items[selected]["text"] as? String ?? "") : "Select") + " ▾", for: .normal)
                button.contentHorizontalAlignment = .leading
                button.menu = UIMenu(children: items.enumerated().map { index, item in
                    let itemID = (item["id"] as? NSNumber)?.int32Value ?? 0
                    return UIAction(title: item["text"] as? String ?? "", state: index == selected ? .on : .off) { [weak self] _ in self?.send { idorid_vm_click($0, itemID) } }
                }); button.showsMenuAsPrimaryAction = true
            default:
                button.setTitle(node["text"] as? String, for: .normal)
                // A plain framework Button (no app background) gets native-looking chrome.
                if type == "Button" && node["draw"] == nil && node["bg"] == nil {
                    button.backgroundColor = UIColor.secondarySystemFill
                    button.layer.cornerRadius = 8
                } else {
                    button.backgroundColor = .clear
                    button.layer.cornerRadius = 0
                }
            }
        }
        if let field = view.widget as? UITextField {
            let text = node["text"] as? String ?? ""; if field.text != text { field.text = text }
            field.placeholder = node["hint"] as? String; field.font = font; field.textColor = color
            field.isSecureTextEntry = (input & 0xfff) == 0x81 || (input & 0xfff) == 0x12 || (input & 0xfff) == 0xe1
            field.keyboardType = keyboardType(input); field.autocapitalizationType = (input & 0xff0) == 0x20 ? .none : .sentences
        }
        if let textView = view.widget as? UITextView {
            let text = node["text"] as? String ?? ""; view.isUpdating = true; if textView.text != text { textView.text = text }; view.isUpdating = false
            textView.font = font; textView.textColor = color; textView.keyboardType = keyboardType(input); textView.isEditable = enabled
            textView.accessibilityHint = node["hint"] as? String
            view.placeholder?.text = node["hint"] as? String; view.placeholder?.font = font; view.placeholder?.isHidden = !text.isEmpty
        }
        if let toggle = view.widget as? UISwitch { toggle.setOn(node["checked"] as? Bool ?? false, animated: false) }
        if let bar = view.widget as? UIProgressView { let maxValue = (node["max"] as? NSNumber)?.floatValue ?? 100; bar.progress = maxValue > 0 ? ((node["progress"] as? NSNumber)?.floatValue ?? 0) / maxValue : 0 }
        if let slider = view.widget as? UISlider { slider.maximumValue = (node["max"] as? NSNumber)?.floatValue ?? 100; slider.value = (node["progress"] as? NSNumber)?.floatValue ?? 0 }
        if let image = view.widget as? UIImageView {
            let src = (node["src"] as? NSNumber)?.int32Value
            image.image = src.flatMap { imageCache[$0] }
            if let src = src, image.image == nil, !missingImages.contains(src) { requestImage(src) }
            if let tint = nodeColor(node["tint"]) { image.image = image.image?.withRenderingMode(.alwaysTemplate); image.tintColor = tint }
            switch node["scaleType"] as? String { case "centerCrop", "center_crop": image.contentMode = .scaleAspectFill; case "fitXY", "fit_xy": image.contentMode = .scaleToFill; case "center": image.contentMode = .center; case "centerInside", "center_inside": image.contentMode = .scaleAspectFit; default: image.contentMode = .scaleAspectFit }
            image.clipsToBounds = true
        }
        if let ops = node["draw"] as? [Any] {
            let canvas = view.canvas ?? CanvasView(frame: view.bounds); view.canvas = canvas
            if canvas.superview == nil { view.insertSubview(canvas, at: 0) }
            canvas.isUserInteractionEnabled = false
            canvas.imageForID = { [weak self] imageID in
                guard let self = self else { return nil }
                if let image = self.imageCache[imageID] { return image }
                if !self.missingImages.contains(imageID) { self.requestImage(imageID) }; return nil
            }
            canvas.updateOperations(ops)
        } else { view.canvas?.removeFromSuperview(); view.canvas = nil }
        let tap = inDialog && enabled && (node["clickable"] as? Bool == true) && !["EditText", "SeekBar", "Spinner"].contains(type)
        let long = enabled && node["longClickable"] as? Bool == true
        let gestureMode = "\(tap)-\(long)"
        if gestureMode != view.gestureMode {
            view.gestureRecognizers?.filter { $0 is ClosureTap || $0 is NodeLongPress }.forEach { view.removeGestureRecognizer($0) }
            if tap { view.addGestureRecognizer(ClosureTap { [weak self] in self?.send { idorid_vm_click($0, id) } }) }
            if long { view.addGestureRecognizer(NodeLongPress { [weak self] in self?.forwardLongClick(id) }) }
            view.gestureMode = gestureMode
        }
        let parent: UIView = (view.scroll as UIView?) ?? view
        let children = node["children"] as? [[String: Any]] ?? []
        for child in children { let sub = build(child, parentKey: key, inDialog: inDialog); if sub.superview !== parent { parent.addSubview(sub) }; parent.bringSubviewToFront(sub) }
        if let scroll = view.scroll {
            let child = children.first.map(nodeFrame) ?? .zero
            scroll.contentSize = CGSize(width: max(0, child.maxX + view.padding.right), height: max(0, child.maxY + view.padding.bottom))
        }
        view.setNeedsLayout()
        return view
    }
    private func keyboardType(_ input: Int) -> UIKeyboardType {
        if (input & 0xff0) == 0x20 { return .emailAddress }
        switch input & 0xf { case 2: return (input & 0x2000) != 0 ? .decimalPad : .numberPad; case 3: return .phonePad; default: return .default }
    }
}

private final class NodeLongPress: UILongPressGestureRecognizer {
    private let callback: () -> Void
    init(_ callback: @escaping () -> Void) { self.callback = callback; super.init(target: nil, action: nil); addTarget(self, action: #selector(fire)) }
    @objc private func fire() { if state == .began { callback() } }
}

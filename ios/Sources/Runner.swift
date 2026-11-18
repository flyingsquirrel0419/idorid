import UIKit

final class RunnerController: UIViewController, UINavigationControllerDelegate {
    private let url: URL
    private let worker = VMWorker()
    private let banner = UILabel()
    private let content = UIView()
    private var fields: [Int32: UITextField] = [:]
    private var closing = false
    private var opened = false
    private var previousDelegate: UINavigationControllerDelegate?

    init(url: URL) { self.url = url; super.init(nibName: nil, bundle: nil) }
    required init?(coder: NSCoder) { fatalError("init(coder:) is unavailable") }
    override func viewDidLoad() {
        super.viewDidLoad()
        title = url.deletingPathExtension().lastPathComponent
        view.backgroundColor = .systemBackground
        navigationItem.hidesBackButton = true
        navigationItem.leftBarButtonItem = UIBarButtonItem(title: "Back", style: .plain, target: self, action: #selector(back))
        navigationItem.rightBarButtonItem = UIBarButtonItem(title: "Log", style: .plain, target: self, action: #selector(showLog))
        let column = UIStackView(arrangedSubviews: [banner, content])
        column.axis = .vertical
        column.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(column)
        NSLayoutConstraint.activate([column.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor), column.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor), column.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor), column.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor)])
        banner.textColor = .white
        banner.backgroundColor = .systemRed
        banner.numberOfLines = 0
        banner.isHidden = true
        worker.open(url) { [weak self] _, error in
            guard let self = self, !self.closing else { return }
            if let error = error { self.showError(error); return }
            self.opened = true
            self.send { idorid_vm_launch($0) }
        }
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
            worker.close()
            navigationController?.delegate = previousDelegate
            navigationController?.interactivePopGestureRecognizer?.isEnabled = true
        }
    }
    private func send(_ action: @escaping (OpaquePointer) -> UnsafeMutablePointer<CChar>?) {
        worker.perform(action) { [weak self] json in
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
    private func render(_ object: [String: Any]) {
        if object["finished"] as? Bool == true { closing = true; worker.close(); navigationController?.popViewController(animated: true); return }
        if let title = object["title"] as? String { self.title = title }
        showError(object["error"] as? String)
        let focused = fields.first { $0.value.isFirstResponder }?.key
        let selection: (Int, Int)? = focused.flatMap { id in
            guard let field = fields[id], let range = field.selectedTextRange else { return nil }
            return (field.offset(from: field.beginningOfDocument, to: range.start), field.offset(from: field.beginningOfDocument, to: range.end))
        }
        fields.removeAll()
        content.subviews.forEach { $0.removeFromSuperview() }
        if let root = object["root"] as? [String: Any] {
            let child = build(root)
            content.addSubview(child)
            child.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([child.leadingAnchor.constraint(equalTo: content.leadingAnchor), child.topAnchor.constraint(equalTo: content.topAnchor)])
            var rootLayout = root
            if rootLayout["width"] == nil { rootLayout["width"] = "match" }
            if rootLayout["height"] == nil { rootLayout["height"] = "match" }
            pinDimension(child, node: rootLayout, horizontal: true, parent: content)
            pinDimension(child, node: rootLayout, horizontal: false, parent: content)
        }
        if let id = focused, let field = fields[id] {
            field.becomeFirstResponder()
            if let selection = selection, let start = field.position(from: field.beginningOfDocument, offset: selection.0), let end = field.position(from: field.beginningOfDocument, offset: selection.1) { field.selectedTextRange = field.textRange(from: start, to: end) }
        }
    }
    private func color(_ value: Any?) -> UIColor? {
        guard let string = value as? String, string.hasPrefix("#"), string.count == 9, let number = UInt32(string.dropFirst(), radix: 16) else { return nil }
        return UIColor(red: CGFloat((number >> 16) & 255) / 255, green: CGFloat((number >> 8) & 255) / 255, blue: CGFloat(number & 255) / 255, alpha: CGFloat((number >> 24) & 255) / 255)
    }
    private func insets(_ node: [String: Any]) -> UIEdgeInsets {
        guard let values = node["padding"] as? [NSNumber], values.count == 4 else { return .zero }
        return UIEdgeInsets(top: CGFloat(values[1].doubleValue), left: CGFloat(values[0].doubleValue), bottom: CGFloat(values[3].doubleValue), right: CGFloat(values[2].doubleValue))
    }
    private func pinDimension(_ child: UIView, node: [String: Any], horizontal: Bool, parent: UIView) {
        let value = node[horizontal ? "width" : "height"]
        let dimension = horizontal ? child.widthAnchor : child.heightAnchor
        if let number = value as? NSNumber { dimension.constraint(equalToConstant: max(0, CGFloat(number.doubleValue))).isActive = true }
        else if value as? String == "match" { dimension.constraint(equalTo: horizontal ? parent.widthAnchor : parent.heightAnchor).isActive = true }
        else { dimension.constraint(lessThanOrEqualTo: horizontal ? parent.widthAnchor : parent.heightAnchor).isActive = true }
    }
    private func build(_ node: [String: Any]) -> UIView {
        let type = node["type"] as? String ?? "View"
        let children = node["children"] as? [[String: Any]] ?? []
        let padding = insets(node)
        let result: UIView
        switch type {
        case "LinearLayout", "ScrollView":
            let stack = UIStackView()
            stack.axis = type == "ScrollView" || (node["orientation"] as? String != "horizontal") ? .vertical : .horizontal
            stack.alignment = .fill
            stack.isLayoutMarginsRelativeArrangement = true
            stack.layoutMargins = padding
            if children.contains(where: { ($0["weight"] as? NSNumber)?.doubleValue ?? 0 > 0 }) {
                stack.distribution = .fillProportionally
            }
            var weighted: [(UIView, CGFloat)] = []
            for childNode in children {
                let child = build(childNode)
                stack.addArrangedSubview(child)
                if let weight = childNode["weight"] as? NSNumber, weight.doubleValue > 0 { weighted.append((child, CGFloat(weight.doubleValue))) }
                for horizontal in [true, false] {
                    let value = childNode[horizontal ? "width" : "height"]
                    let dimension = horizontal ? child.widthAnchor : child.heightAnchor
                    if let number = value as? NSNumber { dimension.constraint(equalToConstant: max(0, CGFloat(number.doubleValue))).isActive = true }
                    else if value as? String == "match" && ((horizontal && stack.axis == .vertical) || (!horizontal && stack.axis == .horizontal)) {
                        dimension.constraint(equalTo: horizontal ? stack.layoutMarginsGuide.widthAnchor : stack.layoutMarginsGuide.heightAnchor).isActive = true
                    }
                }
            }
            if let first = weighted.first {
                for (child, weight) in weighted.dropFirst() {
                    let dimension = stack.axis == .vertical ? child.heightAnchor : child.widthAnchor
                    let reference = stack.axis == .vertical ? first.0.heightAnchor : first.0.widthAnchor
                    let constraint = dimension.constraint(equalTo: reference, multiplier: weight / first.1)
                    constraint.priority = .defaultHigh
                    constraint.isActive = true
                }
            }
            if type == "ScrollView" {
                let scroll = UIScrollView()
                scroll.addSubview(stack)
                stack.translatesAutoresizingMaskIntoConstraints = false
                NSLayoutConstraint.activate([stack.leadingAnchor.constraint(equalTo: scroll.contentLayoutGuide.leadingAnchor), stack.trailingAnchor.constraint(equalTo: scroll.contentLayoutGuide.trailingAnchor), stack.topAnchor.constraint(equalTo: scroll.contentLayoutGuide.topAnchor), stack.bottomAnchor.constraint(equalTo: scroll.contentLayoutGuide.bottomAnchor), stack.widthAnchor.constraint(equalTo: scroll.frameLayoutGuide.widthAnchor)])
                result = scroll
            } else { result = stack }
        case "FrameLayout":
            let frame = UIView()
            for childNode in children {
                let child = build(childNode)
                frame.addSubview(child)
                child.translatesAutoresizingMaskIntoConstraints = false
                NSLayoutConstraint.activate([child.leadingAnchor.constraint(equalTo: frame.leadingAnchor, constant: padding.left), child.topAnchor.constraint(equalTo: frame.topAnchor, constant: padding.top), child.trailingAnchor.constraint(lessThanOrEqualTo: frame.trailingAnchor, constant: -padding.right), child.bottomAnchor.constraint(lessThanOrEqualTo: frame.bottomAnchor, constant: -padding.bottom)])
                for horizontal in [true, false] {
                    let value = childNode[horizontal ? "width" : "height"]
                    let dimension = horizontal ? child.widthAnchor : child.heightAnchor
                    if let n = value as? NSNumber { dimension.constraint(equalToConstant: max(0, CGFloat(n.doubleValue))).isActive = true }
                    else if value as? String == "match" { dimension.constraint(equalTo: horizontal ? frame.widthAnchor : frame.heightAnchor, constant: horizontal ? -padding.left - padding.right : -padding.top - padding.bottom).isActive = true }
                }
            }
            result = frame
        case "TextView":
            let label = UILabel()
            label.numberOfLines = 0
            label.text = node["text"] as? String
            if let size = node["textSize"] as? NSNumber { label.font = .systemFont(ofSize: CGFloat(size.doubleValue)) }
            label.textColor = color(node["color"]) ?? .label
            switch node["gravity"] as? String { case "center": label.textAlignment = .center; case "right": label.textAlignment = .right; default: label.textAlignment = .left }
            result = label
        case "Button":
            let button = UIButton(type: .system)
            button.setTitle(node["text"] as? String, for: .normal)
            if let id = (node["id"] as? NSNumber)?.int32Value { button.addAction(UIAction { [weak self] _ in self?.send { idorid_vm_click($0, id) } }, for: .touchUpInside) }
            result = button
        case "EditText":
            let field = UITextField()
            field.borderStyle = .roundedRect
            field.text = node["text"] as? String
            field.placeholder = node["hint"] as? String
            if let id = (node["id"] as? NSNumber)?.int32Value {
                fields[id] = field
                field.addAction(UIAction { [weak self, weak field] _ in
                    let text = field?.text ?? ""
                    self?.send { vm in text.withCString { idorid_vm_set_text(vm, id, $0) } }
                }, for: .editingChanged)
            }
            result = field
        case "ImageView":
            let placeholder = UIView(); placeholder.backgroundColor = .systemGray4; result = placeholder
        default: result = UIView()
        }
        var outer = result
        if type != "LinearLayout" && type != "ScrollView" && type != "FrameLayout" && padding != .zero {
            let wrapper = UIView()
            wrapper.addSubview(result)
            result.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([
                result.leadingAnchor.constraint(equalTo: wrapper.leadingAnchor, constant: padding.left),
                result.trailingAnchor.constraint(equalTo: wrapper.trailingAnchor, constant: -padding.right),
                result.topAnchor.constraint(equalTo: wrapper.topAnchor, constant: padding.top),
                result.bottomAnchor.constraint(equalTo: wrapper.bottomAnchor, constant: -padding.bottom)
            ])
            outer = wrapper
        }
        if let background = color(node["bg"]) { outer.backgroundColor = background }
        outer.isHidden = (node["visible"] as? Bool) == false
        return outer
    }
}

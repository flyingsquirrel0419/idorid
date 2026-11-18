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
    private var tickTimer: Timer?
    private var presentedDialog: String?
    private let toastLabel = UILabel()

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
            let data = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0].appendingPathComponent("data", isDirectory: true).path
            self.send { vm in
                if let r = data.withCString({ idorid_vm_set_data_dir(vm, $0) }) { idorid_string_free(r) }
                return idorid_vm_launch(vm)
            }
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
            tickTimer?.invalidate()
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
    private func scheduleTick(_ next: Any?) {
        tickTimer?.invalidate()
        guard let ms = (next as? NSNumber)?.doubleValue, ms >= 0 else { return }
        tickTimer = Timer.scheduledTimer(withTimeInterval: max(0.001, ms / 1000), repeats: false) { [weak self] _ in
            self?.send { idorid_vm_tick($0) }
        }
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
            return
        }
        let key = String(describing: dialog)
        if key == presentedDialog { return }
        if presentedViewController is UIAlertController { dismiss(animated: false) }
        presentedDialog = key
        let items = dialog["items"] as? [String] ?? []
        let alert = UIAlertController(title: dialog["title"] as? String, message: dialog["message"] as? String, preferredStyle: items.isEmpty ? .alert : .actionSheet)
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
    private func render(_ object: [String: Any]) {
        if object["finished"] as? Bool == true { closing = true; tickTimer?.invalidate(); worker.close(); navigationController?.popViewController(animated: true); return }
        scheduleTick(object["next"])
        showToasts(object["toasts"] as? [String] ?? [])
        showDialog(object["dialog"] as? [String: Any])
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
        case "HorizontalScrollView":
            let stack = UIStackView(arrangedSubviews: children.map { build($0) })
            stack.axis = .horizontal
            stack.isLayoutMarginsRelativeArrangement = true
            stack.layoutMargins = padding
            let scroll = UIScrollView()
            scroll.addSubview(stack)
            stack.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([stack.leadingAnchor.constraint(equalTo: scroll.contentLayoutGuide.leadingAnchor), stack.trailingAnchor.constraint(equalTo: scroll.contentLayoutGuide.trailingAnchor), stack.topAnchor.constraint(equalTo: scroll.contentLayoutGuide.topAnchor), stack.bottomAnchor.constraint(equalTo: scroll.contentLayoutGuide.bottomAnchor), stack.heightAnchor.constraint(equalTo: scroll.frameLayoutGuide.heightAnchor)])
            result = scroll
        case "Switch", "CheckBox", "RadioButton":
            let row = UIStackView()
            row.axis = .horizontal
            row.spacing = 8
            let checked = node["checked"] as? Bool ?? false
            let id = (node["id"] as? NSNumber)?.int32Value ?? 0
            if type == "Switch" {
                let toggle = UISwitch()
                toggle.isOn = checked
                toggle.addAction(UIAction { [weak self] _ in self?.send { idorid_vm_click($0, id) } }, for: .valueChanged)
                let label = UILabel(); label.text = node["text"] as? String; label.numberOfLines = 0
                row.addArrangedSubview(label); row.addArrangedSubview(toggle)
            } else {
                let button = UIButton(type: .system)
                let symbol = type == "CheckBox" ? (checked ? "checkmark.square.fill" : "square") : (checked ? "largecircle.fill.circle" : "circle")
                button.setImage(UIImage(systemName: symbol), for: .normal)
                button.setTitle("  " + (node["text"] as? String ?? ""), for: .normal)
                button.contentHorizontalAlignment = .leading
                button.addAction(UIAction { [weak self] _ in self?.send { idorid_vm_click($0, id) } }, for: .touchUpInside)
                row.addArrangedSubview(button)
            }
            result = row
        case "ProgressBar":
            if node["indeterminate"] as? Bool ?? true {
                let spinner = UIActivityIndicatorView(style: .medium); spinner.startAnimating(); result = spinner
            } else {
                let bar = UIProgressView(progressViewStyle: .default)
                let max = (node["max"] as? NSNumber)?.floatValue ?? 100
                bar.progress = max > 0 ? ((node["progress"] as? NSNumber)?.floatValue ?? 0) / max : 0
                result = bar
            }
        case "SeekBar":
            let slider = UISlider()
            slider.maximumValue = (node["max"] as? NSNumber)?.floatValue ?? 100
            slider.value = (node["progress"] as? NSNumber)?.floatValue ?? 0
            let id = (node["id"] as? NSNumber)?.int32Value ?? 0
            slider.addAction(UIAction { [weak self, weak slider] _ in
                let value = Int32(slider?.value ?? 0)
                self?.send { idorid_vm_set_progress($0, id, value) }
            }, for: [.touchUpInside, .touchUpOutside])
            result = slider
        case "Spinner":
            let button = UIButton(type: .system)
            let items = node["items"] as? [[String: Any]] ?? []
            let selected = (node["selected"] as? NSNumber)?.intValue ?? -1
            let title = selected >= 0 && selected < items.count ? (items[selected]["text"] as? String ?? "") : "Select"
            button.setTitle(title + " ▾", for: .normal)
            button.contentHorizontalAlignment = .leading
            button.menu = UIMenu(children: items.enumerated().map { index, item in
                let id = (item["id"] as? NSNumber)?.int32Value ?? 0
                return UIAction(title: item["text"] as? String ?? "", state: index == selected ? .on : .off) { [weak self] _ in self?.send { idorid_vm_click($0, id) } }
            })
            button.showsMenuAsPrimaryAction = true
            result = button
        case "ImageButton":
            let button = UIButton(type: .system)
            button.setImage(UIImage(systemName: "photo"), for: .normal)
            if let id = (node["id"] as? NSNumber)?.int32Value { button.addAction(UIAction { [weak self] _ in self?.send { idorid_vm_click($0, id) } }, for: .touchUpInside) }
            result = button
        case "TextView":
            let label = UILabel()
            label.numberOfLines = (node["maxLines"] as? NSNumber)?.intValue ?? 0
            label.text = node["text"] as? String
            if let size = node["textSize"] as? NSNumber {
                let bold = node["bold"] as? Bool ?? false
                var font = UIFont.systemFont(ofSize: CGFloat(size.doubleValue), weight: bold ? .bold : .regular)
                if node["italic"] as? Bool ?? false, let d = font.fontDescriptor.withSymbolicTraits(bold ? [.traitItalic, .traitBold] : .traitItalic) { font = UIFont(descriptor: d, size: 0) }
                label.font = font
            }
            label.textColor = color(node["color"]) ?? .label
            switch node["gravity"] as? String { case "center": label.textAlignment = .center; case "right": label.textAlignment = .right; default: label.textAlignment = .left }
            result = label
        case "Button":
            let button = UIButton(type: .system)
            button.setTitle(node["text"] as? String, for: .normal)
            if let size = node["textSize"] as? NSNumber { button.titleLabel?.font = .systemFont(ofSize: CGFloat(size.doubleValue), weight: node["bold"] as? Bool ?? false ? .bold : .medium) }
            if let c = color(node["color"]) { button.setTitleColor(c, for: .normal) }
            button.isEnabled = node["enabled"] as? Bool ?? true
            if let id = (node["id"] as? NSNumber)?.int32Value { button.addAction(UIAction { [weak self] _ in self?.send { idorid_vm_click($0, id) } }, for: .touchUpInside) }
            result = button
        case "EditText":
            let field = UITextField()
            field.borderStyle = .roundedRect
            field.text = node["text"] as? String
            field.placeholder = node["hint"] as? String
            let inputType = (node["inputType"] as? NSNumber)?.intValue ?? 1
            field.isSecureTextEntry = (inputType & 0xfff) == 0x81 || (inputType & 0xfff) == 0x12 || (inputType & 0xfff) == 0xe1
            switch inputType & 0xf { case 2: field.keyboardType = (inputType & 0x2000) != 0 ? .decimalPad : .numberPad; case 3: field.keyboardType = .phonePad; default: break }
            if (inputType & 0xff0) == 0x20 { field.keyboardType = .emailAddress; field.autocapitalizationType = .none }
            if let id = (node["id"] as? NSNumber)?.int32Value {
                fields[id] = field
                field.addAction(UIAction { [weak self] _ in self?.send { idorid_vm_editor_action($0, id) } }, for: .editingDidEndOnExit)
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
        if let radius = node["radius"] as? NSNumber { outer.layer.cornerRadius = CGFloat(radius.doubleValue); outer.clipsToBounds = true }
        if let alpha = node["alpha"] as? NSNumber { outer.alpha = CGFloat(alpha.doubleValue) }
        outer.isHidden = (node["visible"] as? Bool) == false
        if node["visible"] as? String == "invisible" { outer.alpha = 0 }
        if node["clickable"] as? Bool == true, !["Button", "ImageButton", "EditText", "Switch", "CheckBox", "RadioButton", "SeekBar", "Spinner"].contains(type), let id = (node["id"] as? NSNumber)?.int32Value {
            outer.isUserInteractionEnabled = true
            outer.addGestureRecognizer(ClosureTap { [weak self] in self?.send { idorid_vm_click($0, id) } })
        }
        if let m = node["margin"] as? [NSNumber], m.count == 4, m.contains(where: { $0.doubleValue != 0 }) {
            let wrapper = UIView()
            wrapper.addSubview(outer)
            outer.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([
                outer.leadingAnchor.constraint(equalTo: wrapper.leadingAnchor, constant: CGFloat(m[0].doubleValue)),
                outer.topAnchor.constraint(equalTo: wrapper.topAnchor, constant: CGFloat(m[1].doubleValue)),
                outer.trailingAnchor.constraint(equalTo: wrapper.trailingAnchor, constant: -CGFloat(m[2].doubleValue)),
                outer.bottomAnchor.constraint(equalTo: wrapper.bottomAnchor, constant: -CGFloat(m[3].doubleValue))
            ])
            return wrapper
        }
        return outer
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

import UIKit

/// Replays the Android canvas recording in UIKit's point coordinate system.
final class CanvasView: UIView {
    var imageForID: ((Int32) -> UIImage?)?
    private var operations: [Any] = []
    private var operationsData: Data?

    override init(frame: CGRect) {
        super.init(frame: frame)
        isOpaque = false
        backgroundColor = .clear
        contentMode = .redraw
    }
    required init?(coder: NSCoder) { fatalError("init(coder:) is unavailable") }
    func updateOperations(_ ops: [Any]) {
        let data = try? JSONSerialization.data(withJSONObject: ops, options: [.sortedKeys, .fragmentsAllowed])
        if data == nil || data != operationsData {
            operations = ops
            operationsData = data
            setNeedsDisplay()
        }
    }
    private func number(_ op: [Any], _ i: Int) -> CGFloat? {
        guard i < op.count, let n = op[i] as? NSNumber else { return nil }
        let v = CGFloat(n.doubleValue)
        return v.isFinite ? v : nil
    }
    private func rect(_ op: [Any], _ i: Int) -> CGRect? {
        guard let l = number(op, i), let t = number(op, i+1), let r = number(op, i+2), let b = number(op, i+3), r >= l, b >= t else { return nil }
        return CGRect(x: l, y: t, width: r-l, height: b-t)
    }
    private func color(_ value: Any) -> UIColor? {
        guard let s = value as? String, s.count == 9, s.first == "#", let n = UInt32(s.dropFirst(), radix: 16) else { return nil }
        return UIColor(red: CGFloat((n >> 16) & 255)/255, green: CGFloat((n >> 8) & 255)/255, blue: CGFloat(n & 255)/255, alpha: CGFloat((n >> 24) & 255)/255)
    }
    private func paint(_ op: [Any], _ i: Int, _ context: CGContext, evenOdd: Bool = false) {
        guard i+2 < op.count, let c = color(op[i]), let style = number(op, i+1), let width = number(op, i+2), width >= 0 else { context.beginPath(); return }
        context.setFillColor(c.cgColor); context.setStrokeColor(c.cgColor); context.setLineWidth(width)
        switch style {
        case 0: context.drawPath(using: evenOdd ? .eoFill : .fill)
        case 1: context.drawPath(using: .stroke)
        case 2: context.drawPath(using: evenOdd ? .eoFillStroke : .fillStroke)
        default: context.beginPath()
        }
    }
    private func path(_ string: String) -> CGPath? {
        // Reject unrecognized characters rather than accidentally joining unrelated numbers.
        let pattern = "[MLQCZ]|[-+]?(?:[0-9]+\\.?[0-9]*|\\.[0-9]+)(?:[eE][-+]?[0-9]+)?|[\\s,]+"
        guard let regex = try? NSRegularExpression(pattern: pattern) else { return nil }
        let ns = string as NSString
        let matches = regex.matches(in: string, range: NSRange(location: 0, length: ns.length))
        var tokens: [String] = [], end = 0
        for match in matches {
            guard match.range.location == end else { return nil }
            let token = ns.substring(with: match.range); end = NSMaxRange(match.range)
            if !token.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines.union(CharacterSet(charactersIn: ","))).isEmpty { tokens.append(token) }
        }
        guard end == ns.length else { return nil }
        let result = CGMutablePath()
        var i = 0
        while i < tokens.count {
            let command = tokens[i]; i += 1
            let count: Int
            switch command { case "M", "L": count = 2; case "Q": count = 4; case "C": count = 6; case "Z": count = 0; default: return nil }
            guard i + count <= tokens.count else { return nil }
            var args: [CGFloat] = []
            for token in tokens[i..<(i+count)] {
                guard let n = Double(token), n.isFinite else { return nil }
                args.append(CGFloat(n))
            }
            i += count
            switch command {
            case "M": result.move(to: CGPoint(x: args[0], y: args[1]))
            case "L": result.addLine(to: CGPoint(x: args[0], y: args[1]))
            case "Q": result.addQuadCurve(to: CGPoint(x: args[2], y: args[3]), control: CGPoint(x: args[0], y: args[1]))
            case "C": result.addCurve(to: CGPoint(x: args[4], y: args[5]), control1: CGPoint(x: args[0], y: args[1]), control2: CGPoint(x: args[2], y: args[3]))
            default: result.closeSubpath()
            }
        }
        return result
    }
    override func draw(_ rect: CGRect) {
        guard let ctx = UIGraphicsGetCurrentContext() else { return }
        ctx.setShouldAntialias(true)
        var depth = 0
        for item in operations {
            guard let op = item as? [Any], let name = op.first as? String else { continue }
            switch name {
            case "save": ctx.saveGState(); depth += 1
            case "restore": if depth > 0 { ctx.restoreGState(); depth -= 1 }
            case "translate": if let x = number(op, 1), let y = number(op, 2) { ctx.translateBy(x: x, y: y) }
            case "scale": if let x = number(op, 1), let y = number(op, 2) { ctx.scaleBy(x: x, y: y) }
            case "rotate": if let a = number(op, 1) { ctx.rotate(by: a * .pi / 180) }
            case "clip": if let r = self.rect(op, 1) { ctx.clip(to: r) }
            case "color": if op.count > 1, let c = color(op[1]) { ctx.saveGState(); ctx.setFillColor(c.cgColor); ctx.fill(ctx.boundingBoxOfClipPath); ctx.restoreGState() }
            case "rect", "oval", "rrect":
                guard let r = self.rect(op, 1) else { continue }
                ctx.beginPath()
                if name == "rect" { ctx.addRect(r) }
                else if name == "oval" { ctx.addEllipse(in: r) }
                else {
                    guard let rx = number(op, 5), let ry = number(op, 6), rx >= 0, ry >= 0 else { continue }
                    ctx.addPath(CGPath(roundedRect: r, cornerWidth: rx, cornerHeight: ry, transform: nil))
                }
                paint(op, name == "rrect" ? 7 : 5, ctx)
            case "arc":
                guard let r = self.rect(op, 1), r.width > 0, r.height > 0, let start = number(op, 5), let sweep = number(op, 6), op.count > 7, let center = op[7] as? Bool else { continue }
                let a = start * .pi / 180, delta = max(-360, min(360, sweep)) * .pi / 180
                let p = CGMutablePath()
                if center { p.move(to: .zero); p.addLine(to: CGPoint(x: cos(a), y: sin(a))) }
                p.addArc(center: .zero, radius: 1, startAngle: a, endAngle: a + delta, clockwise: delta < 0)
                if center { p.closeSubpath() }
                // Transform the path, not the stroke width.
                var transform = CGAffineTransform(translationX: r.midX, y: r.midY).scaledBy(x: r.width / 2, y: r.height / 2)
                ctx.beginPath()
                if let transformed = p.copy(using: &transform) { ctx.addPath(transformed); paint(op, 8, ctx) }
            case "line":
                guard let x0 = number(op, 1), let y0 = number(op, 2), let x1 = number(op, 3), let y1 = number(op, 4), op.count > 7, let c = color(op[5]), let width = number(op, 6), width >= 0, let cap = number(op, 7) else { continue }
                ctx.saveGState(); ctx.setStrokeColor(c.cgColor); ctx.setLineWidth(width)
                ctx.setLineCap(cap == 1 ? .round : (cap == 2 ? .square : .butt))
                ctx.beginPath(); ctx.move(to: CGPoint(x: x0, y: y0)); ctx.addLine(to: CGPoint(x: x1, y: y1)); ctx.strokePath(); ctx.restoreGState()
            case "path":
                guard op.count > 5, let s = op[1] as? String, let p = path(s), let fill = number(op, 5) else { continue }
                ctx.beginPath(); ctx.addPath(p); paint(op, 2, ctx, evenOdd: fill == 1)
            case "text":
                guard op.count > 7, let s = op[1] as? String, let x = number(op, 2), let y = number(op, 3), let c = color(op[4]), let size = number(op, 5), size > 0, let align = number(op, 6), let bold = op[7] as? Bool else { continue }
                let font = UIFont.systemFont(ofSize: size, weight: bold ? .bold : .regular)
                let text = NSAttributedString(string: s, attributes: [.font: font, .foregroundColor: c])
                let width = text.size().width
                text.draw(at: CGPoint(x: x - (align == 1 ? width / 2 : (align == 2 ? width : 0)), y: y - font.ascender))
            case "bitmap":
                guard op.count > 6, let n = number(op, 1), n.rounded() == n, n >= CGFloat(Int32.min), n <= CGFloat(Int32.max), let r = self.rect(op, 2), let alpha = number(op, 6), let image = imageForID?(Int32(n)) else { continue }
                image.draw(in: r, blendMode: .normal, alpha: max(0, min(1, alpha / 255)))
            default: break
            }
        }
        while depth > 0 { ctx.restoreGState(); depth -= 1 }
    }
}

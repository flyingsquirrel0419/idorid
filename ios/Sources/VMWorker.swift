import Foundation
import UIKit

// The state and every FFI call are confined to the same 16 MB stack thread.
final class VMWorker {
    final class State {
        let condition = NSCondition()
        var jobs: [(State) -> Void] = []
        var stopping = false
        var vm: OpaquePointer?
        var log = ""

        func take(_ pointer: UnsafeMutablePointer<CChar>?) -> String? {
            guard let pointer = pointer else { return nil }
            defer { idorid_string_free(pointer) }
            return String(cString: pointer)
        }
        func drain() {
            if let vm = vm, let text = take(idorid_vm_take_log(vm)) { log += text }
        }
        func run() {
            while true {
                condition.lock()
                while jobs.isEmpty && !stopping { condition.wait() }
                if jobs.isEmpty && stopping { condition.unlock(); break }
                let job = jobs.removeFirst()
                condition.unlock()
                job(self)
            }
            if let vm = vm { idorid_vm_free(vm) }
        }
    }
    private let state = State()
    private let thread: Thread
    init() {
        let state = self.state
        thread = Thread { state.run() }
        thread.name = "idorid VM"
        thread.stackSize = 16 * 1024 * 1024
        thread.start()
    }
    func submit(_ job: @escaping (State) -> Void) {
        state.condition.lock()
        if !state.stopping { state.jobs.append(job); state.condition.signal() }
        state.condition.unlock()
    }
    func close() {
        state.condition.lock()
        state.stopping = true
        state.condition.signal()
        state.condition.unlock()
    }
    deinit { close() }

    func open(_ url: URL, completion: @escaping (String?, String?) -> Void) {
        submit { state in
            do {
                let data = try Data(contentsOf: url, options: .mappedIfSafe)
                var error: UnsafeMutablePointer<CChar>?
                state.vm = data.withUnsafeBytes { bytes in
                    idorid_vm_new(bytes.bindMemory(to: UInt8.self).baseAddress, bytes.count, &error)
                }
                let message = state.take(error)
                guard let vm = state.vm else {
                    DispatchQueue.main.async { completion(nil, message ?? "Unable to open APK") }
                    return
                }
                let info = state.take(idorid_vm_info(vm))
                state.drain()
                DispatchQueue.main.async { completion(info, nil) }
            } catch {
                DispatchQueue.main.async { completion(nil, error.localizedDescription) }
            }
        }
    }
    func perform(_ action: @escaping (OpaquePointer) -> UnsafeMutablePointer<CChar>?, completion: @escaping (String?) -> Void) {
        submit { state in
            guard let vm = state.vm else { DispatchQueue.main.async { completion(nil) }; return }
            let result = state.take(action(vm))
            state.drain()
            DispatchQueue.main.async { completion(result) }
        }
    }
    /// Decoded drawable images, loaded on the VM thread and cached by resource id.
    func images(_ ids: [Int32], completion: @escaping ([Int32: UIImage]) -> Void) {
        submit { state in
            var out: [Int32: UIImage] = [:]
            if let vm = state.vm {
                for id in ids {
                    var len = 0
                    guard let ptr = idorid_vm_resource_bytes(vm, id, &len) else { continue }
                    let data = Data(bytes: ptr, count: len)
                    idorid_bytes_free(ptr, len)
                    if let image = UIImage(data: data, scale: 3) { out[id] = image }
                }
            }
            DispatchQueue.main.async { completion(out) }
        }
    }
    func logs(_ completion: @escaping (String) -> Void) {
        submit { state in
            state.drain()
            let log = state.log
            DispatchQueue.main.async { completion(log) }
        }
    }
}

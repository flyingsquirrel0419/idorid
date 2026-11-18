package java.lang;

/** Cooperative scheduler for Thread.start(): runs queued threads FIFO. */
public final class VMThread {
    private static Thread[] queue = new Thread[8];
    private static int head, tail;
    private static boolean running;
    private VMThread() {}
    static void enqueue(Thread t) {
        if (tail == queue.length) {
            Thread[] n = new Thread[queue.length * 2];
            System.arraycopy(queue, head, n, 0, tail - head);
            tail -= head; head = 0; queue = n;
        }
        queue[tail++] = t;
    }
    static boolean isPending(Thread t) {
        for (int i = head; i < tail; i++) if (queue[i] == t) return true;
        return false;
    }
    /** Drains all started threads. Called by the VM after each host event. */
    public static void runPending() {
        if (running) return;
        running = true;
        try {
            while (head < tail) {
                Thread t = queue[head];
                queue[head++] = null;
                t.runAsThread();
            }
            head = tail = 0;
        } finally { running = false; }
    }
}

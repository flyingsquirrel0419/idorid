package java.nio.channels.spi;
import java.io.Closeable;
import java.io.IOException;
public abstract class AbstractInterruptibleChannel implements Closeable { private boolean open=true; protected abstract void implCloseChannel() throws IOException; public final boolean isOpen(){return open;} public final void close()throws IOException{if(open){open=false;implCloseChannel();}} protected final void begin(){} protected final void end(boolean completed) throws IOException {} }

package android.content;
public class ContentProviderClient implements java.io.Closeable { public boolean release() { return true; } public void close() {}  public android.database.Cursor query(android.net.Uri uri,String[] projection,String selection,String[] args,String order,android.os.CancellationSignal signal){return null;}
}

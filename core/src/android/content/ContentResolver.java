package android.content;
public class ContentResolver { public ContentProviderClient acquireUnstableContentProviderClient(android.net.Uri uri) { return null; }
 public android.os.ParcelFileDescriptor openFileDescriptor(android.net.Uri uri,String mode,android.os.CancellationSignal signal) throws java.io.FileNotFoundException { throw new java.io.FileNotFoundException(uri.toString()); }
}

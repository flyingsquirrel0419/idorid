package android.os;
import java.io.FileDescriptor;
import java.io.IOException;
public class ParcelFileDescriptor implements Parcelable,java.io.Closeable { private final FileDescriptor descriptor; public ParcelFileDescriptor(FileDescriptor descriptor){this.descriptor=descriptor;} public FileDescriptor getFileDescriptor(){return descriptor;} public int getFd(){return -1;} public void close() throws IOException {} public int describeContents(){return 1;} public void writeToParcel(Parcel p,int flags){p.writeValue(this);} }

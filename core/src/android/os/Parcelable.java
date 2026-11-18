package android.os;
public interface Parcelable {
    int describeContents();
    void writeToParcel(Parcel dest, int flags);
    interface ClassLoaderCreator<T> extends Creator<T> { T createFromParcel(Parcel source, ClassLoader loader); }
    interface Creator<T> { T createFromParcel(Parcel source); T[] newArray(int size); }
}

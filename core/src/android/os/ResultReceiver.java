package android.os;
public class ResultReceiver implements Parcelable {
    private final Handler handler;
    public ResultReceiver(Handler handler) { this.handler=handler; }
    protected void onReceiveResult(int code,Bundle data) {}
    public void send(final int code,final Bundle data) { if(handler==null) onReceiveResult(code,data); else handler.post(new Runnable(){public void run(){onReceiveResult(code,data);}}); }
    public int describeContents() { return 0; } public void writeToParcel(Parcel dest,int flags) { dest.writeValue(this); }
    public static final Parcelable.Creator<ResultReceiver> CREATOR=new Parcelable.Creator<ResultReceiver>() { public ResultReceiver createFromParcel(Parcel p) { return (ResultReceiver)p.readValue(null); } public ResultReceiver[] newArray(int size) { return new ResultReceiver[size]; } };
}

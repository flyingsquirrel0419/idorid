package android.view;
import android.content.ClipData;
import android.net.Uri;
import android.os.Bundle;
public final class ContentInfo {
    public static final int SOURCE_APP=0,SOURCE_CLIPBOARD=1,SOURCE_INPUT_METHOD=2,SOURCE_DRAG_AND_DROP=3,SOURCE_AUTOFILL=4,SOURCE_PROCESS_TEXT=5,FLAG_CONVERT_TO_PLAIN_TEXT=1;
    private final ClipData clip; private final int source,flags; private final Uri link; private final Bundle extras;
    private ContentInfo(Builder b){clip=b.clip;source=b.source;flags=b.flags;link=b.link;extras=b.extras;} public ClipData getClip(){return clip;} public int getSource(){return source;} public int getFlags(){return flags;} public Uri getLinkUri(){return link;} public Bundle getExtras(){return extras;}
    public static final class Builder { private ClipData clip;private int source,flags;private Uri link;private Bundle extras; public Builder(ClipData clip,int source){this.clip=clip;this.source=source;} public Builder(ContentInfo c){clip=c.clip;source=c.source;flags=c.flags;link=c.link;extras=c.extras;} public Builder setClip(ClipData c){clip=c;return this;} public Builder setSource(int s){source=s;return this;} public Builder setFlags(int f){flags=f;return this;} public Builder setLinkUri(Uri u){link=u;return this;} public Builder setExtras(Bundle b){extras=b;return this;} public ContentInfo build(){return new ContentInfo(this);} }
}

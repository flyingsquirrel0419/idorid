package android.view.inputmethod;
import android.net.Uri;
import android.content.ClipDescription;
public final class InputContentInfo { private final Uri uri,link;private final ClipDescription description; public InputContentInfo(Uri uri,ClipDescription description,Uri link){this.uri=uri;this.description=description;this.link=link;} public Uri getContentUri(){return uri;} public Uri getLinkUri(){return link;} public ClipDescription getDescription(){return description;} public void requestPermission(){} public void releasePermission(){} }

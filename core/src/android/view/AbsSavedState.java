package android.view;

import android.os.Parcel;
import android.os.Parcelable;

/** Base class for view saved state, like android.view.AbsSavedState. */
public abstract class AbsSavedState implements Parcelable {
    public static final AbsSavedState EMPTY_STATE = new AbsSavedState() {};
    private final Parcelable superState;
    private AbsSavedState() { superState = null; }
    protected AbsSavedState(Parcelable superState) {
        if (superState == null) throw new IllegalArgumentException("superState must not be null");
        this.superState = superState != EMPTY_STATE ? superState : null;
    }
    protected AbsSavedState(Parcel source) { this(source, null); }
    protected AbsSavedState(Parcel source, ClassLoader loader) { Parcelable s = source == null ? null : source.readParcelable(loader); superState = s != null ? s : EMPTY_STATE; }
    public final Parcelable getSuperState() { return superState; }
    public int describeContents() { return 0; }
    public void writeToParcel(Parcel dest, int flags) { dest.writeParcelable(superState, flags); }
    public static final Parcelable.Creator<AbsSavedState> CREATOR = new Parcelable.ClassLoaderCreator<AbsSavedState>() {
        public AbsSavedState createFromParcel(Parcel in, ClassLoader loader) { Parcelable s = in.readParcelable(loader); if (s != null) throw new IllegalStateException("superState must be null"); return EMPTY_STATE; }
        public AbsSavedState createFromParcel(Parcel in) { return createFromParcel(in, null); }
        public AbsSavedState[] newArray(int n) { return new AbsSavedState[n]; }
    };
}

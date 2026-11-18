package android.view.inputmethod;
import android.os.Bundle;
import android.view.KeyEvent;
public interface InputConnection {
    CharSequence getTextBeforeCursor(int length,int flags); CharSequence getTextAfterCursor(int length,int flags); CharSequence getSelectedText(int flags); int getCursorCapsMode(int reqModes); boolean commitText(CharSequence text,int newCursorPosition); boolean setComposingText(CharSequence text,int newCursorPosition); boolean finishComposingText(); boolean setSelection(int start,int end); boolean deleteSurroundingText(int beforeLength,int afterLength); boolean sendKeyEvent(KeyEvent event); boolean performEditorAction(int actionCode); boolean performContextMenuAction(int id); boolean beginBatchEdit(); boolean endBatchEdit(); boolean performPrivateCommand(String action,Bundle data); boolean commitContent(InputContentInfo content,int flags,Bundle opts);
}

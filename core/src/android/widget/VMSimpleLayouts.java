package android.widget;

import android.content.Context;
import android.view.View;

/** android.R.layout.simple_list_item_1 etc. are framework resources; we build them in code. */
final class VMSimpleLayouts {
    private VMSimpleLayouts() {}
    static boolean isBuiltin(int res) { return (res >>> 24) == 0x01; }
    static View create(Context c, int res) {
        TextView t;
        switch (res) {
            case 0x01090003: // simple_list_item_1
            case 0x01090008: // simple_spinner_item
            case 0x01090009: // simple_spinner_dropdown_item
            case 0x0109000f: // simple_list_item_checked
            case 0x01090010: // simple_list_item_single_choice
            case 0x01090011: // simple_list_item_multiple_choice
            default:
                t = new TextView(c);
                t.setTextSize(res == 0x01090008 ? 16 : 18);
                t.setPadding(16, 12, 16, 12);
                t.setId(0x01020014); // android.R.id.text1
                return t;
        }
    }
}

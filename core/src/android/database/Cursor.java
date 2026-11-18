package android.database;
import android.os.Bundle;
public interface Cursor extends java.io.Closeable {
    int getCount(); int getPosition(); boolean move(int offset); boolean moveToPosition(int position); boolean moveToFirst(); boolean moveToLast(); boolean moveToNext(); boolean moveToPrevious(); boolean isFirst(); boolean isLast(); boolean isBeforeFirst(); boolean isAfterLast();
    int getColumnIndex(String name); int getColumnIndexOrThrow(String name); String getColumnName(int index); String[] getColumnNames(); int getColumnCount(); byte[] getBlob(int index); String getString(int index); short getShort(int index); int getInt(int index); long getLong(int index); float getFloat(int index); double getDouble(int index); int getType(int index); boolean isNull(int index); void close(); boolean isClosed(); Bundle getExtras();
}

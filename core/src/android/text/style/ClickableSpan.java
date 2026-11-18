package android.text.style;
import android.text.TextPaint;
import android.view.View;
public abstract class ClickableSpan extends CharacterStyle { public abstract void onClick(View widget); public void updateDrawState(TextPaint paint){paint.setColor(paint.linkColor);} }

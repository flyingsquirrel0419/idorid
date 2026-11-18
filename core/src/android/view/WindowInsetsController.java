package android.view;
public interface WindowInsetsController { void show(int types); void hide(int types); void setSystemBarsAppearance(int appearance,int mask); int getSystemBarsAppearance(); void setSystemBarsBehavior(int behavior); int getSystemBarsBehavior(); }

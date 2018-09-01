package net.gegy1000.oxide;

public class OxideNative {
    static {
        System.loadLibrary("oxide");
    }

    public static native void loadMod(String name);

    public static native RustModMetadata[] collectMetadata();
}

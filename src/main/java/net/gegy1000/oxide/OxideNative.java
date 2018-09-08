package net.gegy1000.oxide;

import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;

public class OxideNative {
    static {
        System.loadLibrary("oxide");
    }

    public static native void loadMod(String name);

    public static native RustModMetadata[] collectMetadata();

    public static native void dispatchPreInit(int nativeId, FMLPreInitializationEvent event);
}

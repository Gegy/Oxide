package net.gegy1000.oxide;

import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;

public class RustBootstrap {
    static {
        System.loadLibrary("minecraft");
    }

    public static native RustModMetadata constructMod();

    public static native void onPreInit(FMLPreInitializationEvent event);
}

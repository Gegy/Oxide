package net.gegy1000.oxide.core;

import net.gegy1000.oxide.Oxide;
import net.gegy1000.oxide.OxideNative;
import net.gegy1000.oxide.RustModContainer;
import net.gegy1000.oxide.RustModMetadata;
import net.minecraftforge.fml.common.ModContainer;

import java.util.List;

public class OxideHooks {
    public static void identifyMods(List<ModContainer> containers) {
        Oxide.LOGGER.info("Identifying Oxide mods...");

        try {
            OxideNative.loadMod("oxide_example");
        } catch (Throwable t) {
            Oxide.LOGGER.error("Failed to load Rust mod", t);
        }

        RustModMetadata[] metadata = OxideNative.collectMetadata();
        for (RustModMetadata entry : metadata) {
            containers.add(new RustModContainer(entry));
        }
    }
}

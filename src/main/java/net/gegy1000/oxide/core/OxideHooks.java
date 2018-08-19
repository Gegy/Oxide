package net.gegy1000.oxide.core;

import net.gegy1000.oxide.Oxide;
import net.gegy1000.oxide.RustBootstrap;
import net.gegy1000.oxide.RustModContainer;
import net.gegy1000.oxide.RustModMetadata;
import net.minecraftforge.fml.common.ModContainer;

import java.util.List;

public class OxideHooks {
    public static void identifyMods(List<ModContainer> containers) {
        try {
            RustModMetadata metadata = RustBootstrap.constructMod();
            containers.add(new RustModContainer(metadata));
        } catch (Throwable t) {
            Oxide.LOGGER.error("Failed to run Rust mod boostrap", t);
        }
    }
}

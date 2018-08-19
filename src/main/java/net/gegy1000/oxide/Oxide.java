package net.gegy1000.oxide;

import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

@Mod(modid = Oxide.MODID, name = "Oxide", version = Oxide.VERSION, acceptedMinecraftVersions = "[1.12]")
public class Oxide {
    public static final String MODID = "oxide";
    public static final String VERSION = "0.1.0-dev";

    public static final Logger LOGGER = LogManager.getLogger(MODID);

    @Mod.EventHandler
    public static void onPreInit(FMLPreInitializationEvent event) {
        // TODO: Extract the natives into temp dir and add search path for it
        try {
            RustBootstrap.onPreInit(event);
        } catch (Throwable t) {
            LOGGER.error("failed to load dll", t);
        }
    }
}

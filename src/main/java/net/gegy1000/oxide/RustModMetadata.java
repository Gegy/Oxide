package net.gegy1000.oxide;

import net.minecraftforge.fml.common.ModMetadata;

public class RustModMetadata {
    public String modid = "";
    public String name = "";
    public String version = "";

    public ModMetadata createFmlMetadata() {
        ModMetadata meta = new ModMetadata();
        meta.modId = this.modid;
        meta.name = this.name;
        meta.version = this.version;
        return meta;
    }
}

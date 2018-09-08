package net.gegy1000.oxide;

import net.minecraftforge.fml.common.ModMetadata;

public class RustModMetadata {
    public int nativeId = 0;
    public String id = "";
    public String name = "";
    public String version = "";

    public ModMetadata createFmlMetadata() {
        ModMetadata meta = new ModMetadata();
        meta.modId = this.id;
        meta.name = this.name;
        meta.version = this.version;
        return meta;
    }
}

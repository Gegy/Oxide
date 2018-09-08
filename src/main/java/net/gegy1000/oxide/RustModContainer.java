package net.gegy1000.oxide;

import com.google.common.eventbus.EventBus;
import com.google.common.eventbus.Subscribe;
import net.minecraftforge.fml.common.LoadController;
import net.minecraftforge.fml.common.Loader;
import net.minecraftforge.fml.common.MetadataCollection;
import net.minecraftforge.fml.common.ModContainer;
import net.minecraftforge.fml.common.ModMetadata;
import net.minecraftforge.fml.common.event.FMLEvent;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;
import net.minecraftforge.fml.common.versioning.ArtifactVersion;
import net.minecraftforge.fml.common.versioning.DefaultArtifactVersion;
import net.minecraftforge.fml.common.versioning.VersionRange;

import javax.annotation.Nullable;
import java.io.File;
import java.net.URL;
import java.security.cert.Certificate;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class RustModContainer implements ModContainer {
    private final RustModMetadata metadata;
    private final ModMetadata fmlMeta;

    private final RustModInstance instance;

    private int classVersion;
    private ArtifactVersion processedVersion;

    public RustModContainer(RustModMetadata metadata) {
        this.metadata = metadata;
        this.fmlMeta = metadata.createFmlMetadata();

        this.instance = new RustModInstance();
    }

    @Override
    public String getModId() {
        return this.metadata.id;
    }

    @Override
    public String getName() {
        return this.metadata.name;
    }

    @Override
    public String getVersion() {
        return this.metadata.version;
    }

    @Override
    public File getSource() {
        // TODO: Proper source location
        return new File("");
    }

    @Override
    public ModMetadata getMetadata() {
        return this.fmlMeta;
    }

    @Override
    public void bindMetadata(MetadataCollection mc) {
    }

    @Override
    public void setEnabledState(boolean enabled) {
    }

    @Override
    public Set<ArtifactVersion> getRequirements() {
        return Collections.emptySet();
    }

    @Override
    public List<ArtifactVersion> getDependencies() {
        return Collections.emptyList();
    }

    @Override
    public List<ArtifactVersion> getDependants() {
        return Collections.emptyList();
    }

    @Override
    public String getSortingRules() {
        return "";
    }

    @Override
    public boolean registerBus(EventBus bus, LoadController controller) {
        bus.register(this);
        return true;
    }

    @Override
    public boolean matches(Object mod) {
        return this.instance == mod;
    }

    @Override
    public Object getMod() {
        return this.instance;
    }

    @Override
    public ArtifactVersion getProcessedVersion() {
        if (this.processedVersion == null) {
            this.processedVersion = new DefaultArtifactVersion(this.getModId(), this.getVersion());
        }
        return this.processedVersion;
    }

    @Override
    public boolean isImmutable() {
        return false;
    }

    @Override
    public String getDisplayVersion() {
        return this.getVersion();
    }

    @Override
    public VersionRange acceptableMinecraftVersionRange() {
        return Loader.instance().getMinecraftModContainer().getStaticVersionRange();
    }

    @Nullable
    @Override
    public Certificate getSigningCertificate() {
        return null;
    }

    @Override
    public Map<String, String> getCustomModProperties() {
        return null;
    }

    @Override
    public Class<?> getCustomResourcePackClass() {
        return null;
    }

    @Override
    public Map<String, String> getSharedModDescriptor() {
        return Collections.emptyMap();
    }

    @Override
    public Disableable canBeDisabled() {
        return Disableable.NEVER;
    }

    @Override
    public String getGuiClassName() {
        return null;
    }

    @Override
    public List<String> getOwnedPackages() {
        return Collections.emptyList();
    }

    @Override
    public boolean shouldLoadInEnvironment() {
        return true;
    }

    @Override
    public URL getUpdateUrl() {
        return null;
    }

    @Override
    public void setClassVersion(int classVersion) {
        this.classVersion = classVersion;
    }

    @Override
    public int getClassVersion() {
        return this.classVersion;
    }

    @Subscribe
    public void handleModStateEvent(FMLEvent event) {
        if (event instanceof FMLPreInitializationEvent) {
            OxideNative.dispatchPreInit(this.metadata.nativeId, (FMLPreInitializationEvent) event);
        }
    }
}

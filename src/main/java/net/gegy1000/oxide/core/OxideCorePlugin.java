package net.gegy1000.oxide.core;

import com.google.common.reflect.Reflection;
import net.gegy1000.oxide.OxideNative;
import net.minecraftforge.fml.relauncher.IFMLLoadingPlugin;

import javax.annotation.Nullable;
import java.util.Map;

@IFMLLoadingPlugin.Name("oxide")
@IFMLLoadingPlugin.MCVersion("1.12.2")
@IFMLLoadingPlugin.SortingIndex(1001)
public class OxideCorePlugin implements IFMLLoadingPlugin {
    @Override
    public String[] getASMTransformerClass() {
        return new String[] { "net.gegy1000.oxide.core.OxideClassTransformer" };
    }

    @Override
    public String getModContainerClass() {
        return null;
    }

    @Nullable
    @Override
    public String getSetupClass() {
        return null;
    }

    @Override
    public void injectData(Map<String, Object> data) {
        Reflection.initialize(OxideNative.class);
    }

    @Override
    public String getAccessTransformerClass() {
        return null;
    }
}

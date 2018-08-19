package net.gegy1000.oxide.core;

import net.minecraft.launchwrapper.IClassTransformer;
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.Opcodes;
import org.objectweb.asm.tree.AbstractInsnNode;
import org.objectweb.asm.tree.ClassNode;
import org.objectweb.asm.tree.InsnList;
import org.objectweb.asm.tree.MethodInsnNode;
import org.objectweb.asm.tree.MethodNode;
import org.objectweb.asm.tree.VarInsnNode;

import java.util.ListIterator;

public class OxideClassTransformer implements IClassTransformer {
    private static final String HOOKS_CLASS = "net/gegy1000/oxide/core/OxideHooks";

    @Override
    public byte[] transform(String name, String transformedName, byte[] bytes) {
        if (bytes == null) {
            return null;
        }
        if (name.equals("net.minecraftforge.fml.common.discovery.ModDiscoverer")) {
            ClassNode node = this.parse(bytes);
            return this.hookModDiscoverer(node);
        }
        return bytes;
    }

    private byte[] hookModDiscoverer(ClassNode node) {
        for (MethodNode method : node.methods) {
            if (method.name.equals("identifyMods")) {
                InsnList instructions = new InsnList();
                ListIterator<AbstractInsnNode> iterator = method.instructions.iterator();
                while (iterator.hasNext()) {
                    AbstractInsnNode instruction = iterator.next();
                    instructions.add(instruction);
                    if (instruction instanceof VarInsnNode) {
                        if (instruction.getOpcode() == Opcodes.ASTORE && ((VarInsnNode) instruction).var == 1) {
                            instructions.add(new VarInsnNode(Opcodes.ALOAD, 1));
                            instructions.add(new MethodInsnNode(Opcodes.INVOKESTATIC, HOOKS_CLASS, "identifyMods", "(Ljava/util/List;)V", false));
                        }
                    }
                }
                method.instructions.clear();
                method.instructions.add(instructions);
            }
        }
        return this.write(node);
    }

    private ClassNode parse(byte[] bytes) {
        ClassReader reader = new ClassReader(bytes);
        ClassNode node = new ClassNode();
        reader.accept(node, 0);
        return node;
    }

    private byte[] write(ClassNode node) {
        ClassWriter writer = new ClassWriter(ClassWriter.COMPUTE_MAXS);
        node.accept(writer);
        return writer.toByteArray();
    }
}

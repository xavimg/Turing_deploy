package org.proj.db.codecs.primitive;

import org.proj.data.cross.CrossCodec;
import org.proj.data.cross.read.CrossReader;
import org.proj.data.cross.read.ValueType;
import org.proj.data.cross.write.CrossWriter;

import java.lang.reflect.Array;
import java.util.ArrayList;

public class ArrayCodec<T> implements CrossCodec<T[]> {
    final private CrossCodec<T> codec;

    public ArrayCodec (CrossCodec<T> codec) {
        this.codec = codec;
    }

    @Override
    public T[] decode (CrossReader reader) {
        ArrayList<T> list = new ArrayList<>();

        reader.readStartArray();
        while (reader.nextValueType() != ValueType.END) {
            list.add(codec.decode(reader));
        }
        reader.readEndArray();

        T[] target = (T[]) Array.newInstance(codec.getEncoderClass(), list.size());
        return list.toArray(target);
    }

    @Override
    public void encode (CrossWriter writer, T[] value) {
        writer.writeStartArray();
        for (T elem: value) {
            codec.encode(writer, elem);
        }
        writer.writeEndArray();
    }

    @Override
    public Class<T[]> getEncoderClass() {
        T[] dummy = (T[]) Array.newInstance(codec.getEncoderClass(), 0);
        return (Class<T[]>) dummy.getClass();
    }
}

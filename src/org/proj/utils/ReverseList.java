package org.proj.utils;

import org.rol.ReadOnlyList;

import java.util.List;

public class ReverseList<T> extends ReadOnlyList<T> {
    final private List<T> parent;

    public ReverseList (List<T> parent) {
        this.parent = parent;
    }

    @Override
    public T get (int i) {
        return parent.get(size() - i);
    }

    @Override
    public int size() {
        return parent.size();
    }
}

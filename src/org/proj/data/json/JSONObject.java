package org.proj.data.json;

import com.mongodb.util.JSON;
import org.proj.utils.Range;
import org.proj.utils.SafeReader;

import java.io.Reader;
import java.lang.reflect.Array;
import java.util.*;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicReference;
import java.util.stream.Collectors;

public class JSONObject {
    final private HashMap<String, Object> data;

    public JSONObject() {
        this.data = new HashMap<>();
    }

    private JSONObject (SafeReader safe, boolean skipFirst) {
        if (skipFirst) safe.skipWhile(true, c -> c != '{');
        this.data = new HashMap<>();

        // LOOP
        while (safe.hasNext()) {
            Optional<Character> last = safe.skipWhile(true, c -> c != '"');
            if (last.isEmpty() || last.get() == '}') {
                break;
            }

            String key = safe.joinWhile(true, c -> c != '"');
            safe.skipWhile(true, c -> c != ':');
            Object value = readValue(safe);

            this.data.put(key, value);
        }
    }

    public JSONObject (SafeReader safe) {
        this(safe, true);
    }

    public JSONObject (Reader reader) {
        this(new SafeReader(reader));
    }

    private static Object readValue (SafeReader safe) {
        char last = safe.skipWhile(true, c -> !Character.isLetterOrDigit(c) & c != '.' & c != '"' & c != '{' & c != '[' & c != ']' & c != '}').get();

        // NUMBER
        if (Character.isDigit(last) || last == '.') {
            AtomicBoolean hasDecimal = new AtomicBoolean(last == '.');
            StringBuilder builder = new StringBuilder().append(last);

            safe.forEachWhile(false, c -> Character.isDigit(c) || (!hasDecimal.get() && c == '.'), c -> {
                if (c == '.') hasDecimal.set(true);
                builder.append(c);
            });

            return new UnparsedNumber(builder.toString());
        }

        // BOOL
        else if (last == 't' || last == 'f') {
            String next = last + safe.joinWhile(false, Character::isLetter);
            return Boolean.parseBoolean(next);
        }

        // STRING
        else if (last == '"') {
            return safe.joinWhile(true, c -> c != '"');
        }

        // NULL
        else if (last == 'n') {
            String nul = last + safe.joinWhile(false, Character::isLetter);
            if (!nul.equals("null")) throw new RuntimeException("Parse error");
            return Optional.empty();
        }

        // ARRAY / LIST
        else if (last == '[') {
            ArrayList<Object> list = new ArrayList<>();
            while (safe.hasNext()) {
                Object value = readValue(safe);
                if (value == null) break;
                list.add(value);
            }

            // CHECK FOR SAME TYPE
            AtomicReference<Class<?>> first = new AtomicReference<>(list.get(0).getClass());
            boolean sameType = Range.ofIterable(list, true).skip(1).allMatch(x -> {
                Class<?> clazz = x.getClass();
                boolean isChild = first.get().isAssignableFrom(clazz);
                if (isChild) return true;

                boolean isParent = clazz.isAssignableFrom(first.get());
                if (isParent) {
                    first.set(clazz);
                    return true;
                }

                return false;
            });

            if (sameType) {
                Object array = Array.newInstance(first.get(), list.size());
                Range.ofInt(0, list.size(), true).forEach(i -> Array.set(array, i, list.get(i)));
                return new ArrayWrapper(first.get(), array);
            } else {
                return list; // TODO?
            }
        }

        else if (last == ']') {
            return null;
        }

        // OBJECT
        else if (last == '{') {
            return new JSONObject(safe, false);
        }

        else if (last == '}') {
            return null;
        }

        throw new RuntimeException("No valid value found");
    }

    // GETTERS
    public boolean getBool (String name) {
        return (Boolean) data.get(name);
    }

    private Number getNumber (String name) {
        return (Number) data.get(name);
    }

    public int getInt (String name) {
        return getNumber(name).intValue();
    }

    public long getLong (String name) {
        return getNumber(name).longValue();
    }

    public float getFloat (String name) {
        return getNumber(name).floatValue();
    }

    public double getDouble (String name) {
        return getNumber(name).doubleValue();
    }

    public String getString (String name) {
        return data.get(name).toString();
    }

    public JSONObject getObject (String name) {
        return (JSONObject) data.get(name);
    }

    private ArrayWrapper<?> getArray (String name) {
        return (ArrayWrapper<?>) data.get(name);
    }

    public boolean[] getBoolArray (String name) {
        return (boolean[]) getArray(name).array;
    }

    public int[] getIntArray (String name) {
        return (int[]) getArray(name).array;
    }

    public long[] getLongArray (String name) {
        return (long[]) getArray(name).array;
    }

    public float[] getFloatArray (String name) {
        return (float[]) getArray(name).array;
    }

    public double[] getDoubleArray (String name) {
        return (double[]) getArray(name).array;
    }

    public String[] getStringArray (String name) {
        return (String[]) getArray(name).array;
    }

    public JSONObject[] getObjectArray (String name) {
        return (JSONObject[]) getArray(name).array;
    }

    // SETTERS
    public JSONObject put (String name, boolean value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, int value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, long value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, float value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, double value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, String value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, JSONObject value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, boolean... value) {
        data.put(name, new ArrayWrapper<>(Boolean.TYPE, value));
        return this;
    }

    public JSONObject put (String name, int... value) {
        data.put(name, new ArrayWrapper<>(Integer.TYPE, value));
        return this;
    }

    public JSONObject put (String name, long... value) {
        data.put(name, new ArrayWrapper<>(Long.TYPE, value));
        return this;
    }

    public JSONObject put (String name, float... value) {
        data.put(name, new ArrayWrapper<>(Float.TYPE, value));
        return this;
    }

    public JSONObject put (String name, double... value) {
        data.put(name, new ArrayWrapper<>(Double.TYPE, value));
        return this;
    }

    public JSONObject put (String name, String... value) {
        data.put(name, value);
        return this;
    }

    public JSONObject put (String name, JSONObject... value) {
        data.put(name, new ArrayWrapper<>(JSONObject.class, value));
        return this;
    }

    public JSONObject putNull (String name) {
        data.put(name, null);
        return this;
    }

    // toString
    public String toJsonString () {
        return '{' + Range.ofIterable(data.entrySet(), true)
                .map(x -> '"' + x.getKey() + "\":" + mapValue(x.getValue()))
                .collect(Collectors.joining(",")) + '}';
    }

    private static String mapValue (Object value) {
        if (value == null) {
            return "null";
        } else if (value instanceof String) {
            return '"' + (String) value + '"';
        } else if (value instanceof ArrayWrapper) {
            return '[' + Range.ofIterable((ArrayWrapper<?>) value, true)
                    .map(JSONObject::mapValue)
                    .collect(Collectors.joining(",")) + ']';
        } else if (value instanceof JSONObject) {
            return ((JSONObject) value).toJsonString();
        }

        return value.toString();
    }

    // SUBCLASSES
    private static class UnparsedNumber extends Number {
        final String value;

        public UnparsedNumber(String value) {
            this.value = value;
        }

        @Override
        public byte byteValue() {
            return Byte.parseByte(value);
        }

        @Override
        public short shortValue() {
            return Short.parseShort(value);
        }

        @Override
        public int intValue() {
            return Integer.parseInt(value);
        }

        @Override
        public long longValue() {
            return Long.parseLong(value);
        }

        @Override
        public float floatValue() {
            return Float.parseFloat(value);
        }

        @Override
        public double doubleValue() {
            return Double.parseDouble(value);
        }
    }

    private static class ArrayWrapper<T> implements Iterable<T> {
        Class<T> type;
        Object array;

        public ArrayWrapper (Class<T> type, Object array) {
            this.type = type;
            this.array = array;
        }

        @Override
        public Iterator<T> iterator() {
            return new Iterator<T>() {
                int i = 0;

                @Override
                public boolean hasNext () {
                    return i < Array.getLength(array);
                }

                @Override
                public T next() {
                    return (T) Array.get(array, i++);
                }
            };
        }

        @Override
        public Spliterator<T> spliterator() {
            return Spliterators.spliterator(this.iterator(), Array.getLength(array), 0);
        }
    }
}

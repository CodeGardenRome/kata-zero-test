package lambdaroma;

import lambdaroma.katazero.KataZeroStream;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import java.util.*;

import static org.junit.jupiter.api.Assertions.assertEquals;

@Tag("fast")
public class KataZeroTest {

    KataZeroStream daVerificare = new KataZeroStream();

    @Test
    void kataZeroA(){
        assertEquals(Arrays.asList(1,2,3), daVerificare.kataZeroA());
    }

    @Test
    void kataZeroB(){
        assertEquals(Arrays.asList(2,4,6,8,10), daVerificare.kataZeroB());
    }

    @Test
    void kataZeroC(){
        assertEquals(Arrays.asList(0, 7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91, 98), daVerificare.kataZeroC());
    }

    @Test
    void kataZeroD(){
        assertEquals(Arrays.asList("Caio","Calpurnio"), daVerificare.kataZeroD());
    }

    @Test
    void kataZeroE(){
        assertEquals(new Double(52), daVerificare.kataZeroE());
    }

    @Test
    void kataZeroF(){
        assertEquals(83166, daVerificare.kataZeroF());
    }

    @Test
    void kataZeroG(){
        assertEquals(Arrays.asList("Caio","Calpurnio","Filano", "Mevio", "Sempronio", "Tizio"),
                daVerificare.kataZeroG());
    }

    @Test
    void kataZeroH(){
        Assertions.assertTrue(daVerificare.kataZeroH() % 41 == 0);
    }

    @Test
    void kataZeroI(){
        assertEquals("Tizio, Caio, Sempronio, Mevio, Filano, Calpurnio",
                daVerificare.kataZeroI());
    }

    @Test
    void kataZeroJ(){
        assertEquals(new HashSet<>(Arrays.asList("Tizio", "Caio", "Sempronio", "Mevio", "Filano", "Calpurnio")),
                daVerificare.kataZeroJ());
    }

    @Test
    void kataZeroK(){
        Map<Integer,List<String>> lunghezzeNomi = new HashMap<>();
        lunghezzeNomi.put(4, Arrays.asList("Anna","Emma","Sara"));
        lunghezzeNomi.put(5, Arrays.asList("Carla","Maria"));
        lunghezzeNomi.put(6, Arrays.asList("Angela","Chiara"));
        assertEquals(lunghezzeNomi, daVerificare.kataZeroK());
    }

    @Test
    void kataZeroL() {
        assertEquals(Arrays.asList(4, 5, 6, 6, 4, 5, 4), daVerificare.kataZeroL());
    }

    @Test
    void kataZeroM() {
        assertEquals(Arrays.asList("A","C","A","C","E","M","S"), daVerificare.kataZeroM());
    }

}

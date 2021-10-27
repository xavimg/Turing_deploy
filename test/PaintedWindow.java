import javax.swing.*;
import java.awt.*;

public abstract class PaintedWindow extends JFrame {
    final private PaintedWindowPanel panel;

    public PaintedWindow(String title) throws HeadlessException {
        super(title);

        this.panel = new PaintedWindowPanel();
        this.add(this.panel);
    }

    public abstract void paint (Graphics g);

    @Override
    public void setSize (int width, int height) {
        this.panel.setSize(width, height);
        super.setSize(width, height);
    }

    @Override
    public void repaint() {
        super.repaint();
        panel.repaint();
    }

    public class PaintedWindowPanel extends Canvas {
        @Override
        public void paint (Graphics g) {
            super.paint(g);
            PaintedWindow.this.paint(g);
        }
    }
}

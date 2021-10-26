import javax.swing.*;
import java.awt.*;

public abstract class PaintedWindow extends JFrame {
    final private PaintedWindowPanel panel;

    public PaintedWindow(String title) throws HeadlessException {
        super(title);
        this.setLayout(new FlowLayout());

        this.panel = new PaintedWindowPanel();
        this.add(this.panel);
    }

    public abstract void paint (Graphics g);

    public class PaintedWindowPanel extends JPanel {
        public PaintedWindowPanel() {
            super(true);
        }

        @Override
        protected void paintComponent (Graphics g) {
            PaintedWindow.this.paint(g);
        }
    }
}

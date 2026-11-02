#!/usr/bin/env python3
"""Generate chapter 4 SVGs with actual cell sizes and an undistorted viewBox."""
from pathlib import Path
from html import escape

DEST = Path(__file__).resolve().parents[1] / 'src/img'
INK = '#17324d'
BLUE = '#2563eb'
ORANGE = '#c65d24'

class Diagram:
    def __init__(self, width, height, title, description):
        self.width, self.height = width, height
        self.parts = [f'''<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" preserveAspectRatio="xMidYMid meet" role="img" aria-labelledby="title desc">
<title id="title">{escape(title)}</title>
<desc id="desc">{escape(description)}</desc>
<defs><marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M 0 1 L 9 5 L 0 9 Z" fill="{BLUE}"/></marker></defs>
<style>text {{ font-family: "DejaVu Sans", sans-serif; fill: {INK}; font-size: 14px; }} .label {{ font-size: 12px; font-weight: 600; letter-spacing: 1px; }} .code {{ font-family: "DejaVu Sans Mono", monospace; }} .heading {{ font-weight: 600; }} .grid {{ stroke: #cbd5e1; stroke-width: 1; }}</style>
<rect x="1" y="1" width="{width-2}" height="{height-2}" rx="16" fill="#fff" stroke="#e2e8f0"/>''']
    def text(self, x, y, value, cls='', anchor='middle', color=None):
        fill = f' style="fill:{color}"' if color else ''
        self.parts.append(f'<text x="{x}" y="{y}" text-anchor="{anchor}" dominant-baseline="middle" class="{cls}"{fill}>{escape(str(value))}</text>')
    def label(self, x, y, value, color=BLUE):
        self.text(x, y, value, 'label', 'middle', color)
    def table(self, x, y, title, headers, rows, widths=(112, 72), muted=False):
        w, h = sum(widths), 34 * (len(rows)+1)
        accent = '#64748b' if muted else (ORANGE if headers[0]=='indeks' else BLUE)
        bg = '#f1f5f9' if muted else ('#fff7ed' if headers[0]=='indeks' else '#eff6ff')
        if title:
            self.text(x+w/2, y-17, title, 'code heading', color=accent)
        self.parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="8" fill="{bg}" stroke="#cbd5e1"/>')
        self.parts.append(f'<path d="M{x},{y+34} H{x+w} M{x+widths[0]},{y} V{y+h}" class="grid" fill="none"/>')
        for i in range(2, len(rows)+1):
            self.parts.append(f'<path d="M{x},{y+34*i} H{x+w}" class="grid"/>')
        for i, value in enumerate(headers):
            self.text(x+sum(widths[:i])+widths[i]/2, y+17, value, 'heading', color=accent)
        for j, row in enumerate(rows):
            for i, value in enumerate(row):
                self.text(x+sum(widths[:i])+widths[i]/2, y+51+j*34, value, 'code' if i else '')
        if muted:
            self.parts.append(f'<path d="M{x+8},{y+h-8} L{x+w-8},{y+8}" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 5"/>')
        return (x+widths[0]+widths[1]/2, y+51), (x,y+51)
    def stack(self, x, y, name, length=5, capacity=None, muted=False, pointer_only=False):
        rows=[('işaretçi','')]
        if not pointer_only:
            rows += [('uzunluk',length), ('kapasite',length if capacity is None else capacity)]
        return self.table(x,y,name,('ad','değer'),rows,muted=muted)[0]
    def heap(self, x, y, value='hello', muted=False):
        return self.table(x,y,'',('indeks','değer'),[(i,c if c!=' ' else '␣') for i,c in enumerate(value)],(72,72),muted)[1]
    def arrow(self, start, end, bend=False, label=None, label_position=None):
        x,y=start; X,Y=end
        path=f'M{x},{y} C{x+55},{y} {X-55},{Y} {X-4},{Y}' if bend else f'M{x},{y} L{X-4},{Y}'
        self.parts.append(f'<circle cx="{x}" cy="{y}" r="3" fill="{BLUE}"/><path d="{path}" fill="none" stroke="{BLUE}" stroke-width="2" marker-end="url(#arrow)"/>')
        if label:
            label_x, label_y = label_position or ((x+X)/2, (y+Y)/2-16)
            self.text(label_x,label_y,label,'',color=BLUE)
    def save(self, number):
        (DEST / f'trpl04-{number:02}.svg').write_text('\n'.join(self.parts+['</svg>'])+'\n')

def main():
    descriptions = {
        1: ('String bellekte nasıl tutulur?', 's1 değişkeninin işaretçisi öbekteki hello verisine gider. Uzunluk ve kapasite 5 bayttır.'),
        2: ('Sahipliğin taşınması', 's1 ve s2 aynı öbek verisine işaret eder. Taşıma sonrasında s1 artık geçerli değildir; geçersizlik bir sonraki şekilde gösterilir.'),
        3: ('Veriyi klonlama', 's1 ve s2, hello verisinin öbekteki iki ayrı kopyasına işaret eder.'),
        4: ('Taşıma sonrası geçersiz değişken', 's1 geçersizdir. Aynı öbek verisinin yeni sahibi s2 değişkenidir.'),
        5: ('String değerini yeniden atama', 's artık dört baytlık ahoy verisine işaret eder. Önceki hello verisi serbest bırakılmıştır.'),
        6: ('String değerine referans', 's referansı s1 değişkenini ödünç alır. s1, öbekteki hello verisinin sahibidir.'),
        7: ('String dilimi', 's, hello world verisinin tamamına işaret eder. world dilimi 6. bayttan başlar ve 5 bayt uzunluğundadır. ␣ boşluk karakterini gösterir.'),
    }
    for n,(title,desc) in descriptions.items():
        width=860 if n==6 else 640
        height=580 if n==7 else (580 if n==5 else 550 if n==3 else 470 if n in (2,3,4) else 330)
        d=Diagram(width,height,title,desc)
        if n==6:
            d.label(120,58,'REFERANS');d.label(464,58,'YIĞIN');d.label(756,58,'ÖBEK',ORANGE)
            a=d.stack(28,100,'s',pointer_only=True);b=d.stack(372,100,'s1');c=d.heap(684,100)
            d.arrow(a,(372,151),label='ödünç alır',label_position=(292,132));d.arrow(b,c)
        else:
            d.label(124,58,'YIĞIN');d.label(504,58,'ÖBEK',ORANGE)
            a=d.stack(32,100,'s' if n in (5,7) else 's1',11 if n==7 else 4 if n==5 else 5,muted=n==4)
            c=d.heap(432,100,'hello world' if n==7 else 'ahoy' if n==5 else 'hello')
            d.arrow(a,c)
            if n in (2,4):
                b=d.stack(32,310,'s2');d.arrow(b,c,bend=True)
            elif n==3:
                b=d.stack(32,310,'s2');e=d.heap(432,310);d.arrow(b,e)
            elif n==5:
                d.label(504,315,'SERBEST BIRAKILDI', '#64748b');d.heap(432,345,'hello',muted=True)
            elif n==7:
                b=d.table(32,345,'world',('ad','değer'),[('işaretçi',''),('uzunluk',5)])[0]
                d.arrow(b,(432,151+6*34),bend=True)
                d.text(32,548,'␣ = boşluk · Dilim: &s[6..11]','','start', '#64748b')
        d.save(n)

if __name__ == '__main__':
    main()

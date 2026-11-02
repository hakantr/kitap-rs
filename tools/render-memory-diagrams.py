#!/usr/bin/env python3
"""Generate chapter 4 and String/Pin SVGs with actual cell sizes and an undistorted viewBox."""
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
    def stack(self, x, y, name, length=7, capacity=None, muted=False, pointer_only=False):
        rows=[('işaretçi','')]
        if not pointer_only:
            rows += [('uzunluk',length), ('kapasite',length if capacity is None else capacity)]
        return self.table(x,y,name,('ad','değer'),rows,muted=muted)[0]
    def heap(self, x, y, value='merhaba', muted=False):
        # One cell is one byte in these ASCII examples.
        assert value.isascii()
        return self.table(x,y,'',('indeks','değer'),[(i,c if c!=' ' else '␣') for i,c in enumerate(value)],(72,72),muted)[1]
    def arrow(self, start, end, bend=False, label=None, label_position=None):
        x,y=start; X,Y=end
        path=f'M{x},{y} C{x+55},{y} {X-55},{Y} {X-4},{Y}' if bend else f'M{x},{y} L{X-4},{Y}'
        self.parts.append(f'<circle cx="{x}" cy="{y}" r="3" fill="{BLUE}"/><path d="{path}" fill="none" stroke="{BLUE}" stroke-width="2" marker-end="url(#arrow)"/>')
        if label:
            label_x, label_y = label_position or ((x+X)/2, (y+Y)/2-16)
            self.text(label_x,label_y,label,'',color=BLUE)
    def save(self, number, chapter=4):
        (DEST / f'trpl{chapter:02}-{number:02}.svg').write_text('\n'.join(self.parts+['</svg>'])+'\n')

def main():
    descriptions = {
        1: ('String bellekte nasıl tutulur?', 'metin1 değişkeninin işaretçisi öbekteki merhaba verisine gider. Uzunluk ve kapasite 7 bayttır.'),
        2: ('Sahipliğin taşınması', 'metin1 ve metin2 aynı öbek verisine işaret eder. Taşıma sonrasında metin1 artık geçerli değildir; geçersizlik bir sonraki şekilde gösterilir.'),
        3: ('Öbek verisinin ayrı kopyaları', 'metin1 ve metin2, merhaba verisinin öbekteki iki ayrı kopyasına işaret eder.'),
        4: ('Taşıma sonrası geçersiz değişken', 'metin1 geçersizdir. Aynı öbek verisinin yeni sahibi metin2 değişkenidir.'),
        5: ('String değerini yeniden atama', 'metin artık beş baytlık selam verisine işaret eder. Önceki merhaba verisi serbest bırakılmıştır.'),
        6: ('String değerine referans', 'metin referansı metin1 değişkenini ödünç alır. metin1, öbekteki merhaba verisinin sahibidir.'),
        7: ('String dilimi', 'metin, 13 baytlık merhaba dunya verisinin tamamına işaret eder. dunya dilimi 8. bayttan başlar ve 5 bayt uzunluğundadır. ␣ boşluk karakterini gösterir.'),
    }
    for n,(title,desc) in descriptions.items():
        width=860 if n==6 else 640
        height={1:400, 2:520, 3:730, 4:520, 5:700, 6:400, 7:660}[n]
        d=Diagram(width,height,title,desc)
        if n==6:
            d.label(120,58,'REFERANS');d.label(464,58,'YIĞIN');d.label(756,58,'ÖBEK',ORANGE)
            a=d.stack(28,100,'metin',pointer_only=True);b=d.stack(372,100,'metin1');c=d.heap(684,100)
            d.arrow(a,(372,151),label='ödünç alır',label_position=(292,132));d.arrow(b,c)
        else:
            d.label(124,58,'YIĞIN');d.label(504,58,'ÖBEK',ORANGE)
            a=d.stack(32,100,'metin' if n in (5,7) else 'metin1',13 if n==7 else 5 if n==5 else 7,muted=n==4)
            c=d.heap(432,100,'merhaba dunya' if n==7 else 'selam' if n==5 else 'merhaba')
            d.arrow(a,c)
            if n in (2,4):
                b=d.stack(32,350,'metin2');d.arrow(b,c,bend=True)
            elif n==3:
                b=d.stack(32,430,'metin2');e=d.heap(432,430);d.arrow(b,e)
            elif n==5:
                d.label(504,360,'SERBEST BIRAKILDI', '#64748b');d.heap(432,390,'merhaba',muted=True)
            elif n==7:
                b=d.table(32,385,'dunya',('ad','değer'),[('işaretçi',''),('uzunluk',5)])[0]
                d.arrow(b,(432,151+8*34),bend=True)
                d.text(32,628,'␣ = boşluk · Dilim: &metin[8..13]','','start', '#64748b')
        d.save(n)

def pin_diagrams():
    def panel(d, x, y, width, height):
        d.parts.append(f'<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="10" fill="#eff6ff" stroke="#94a3b8" stroke-dasharray="6 4"/>')
        d.text(x+width/2,y+24,'String','heading')
    def string_row(d, x, y, name, value, muted=False):
        # These examples deliberately use ASCII: each visible cell is one byte.
        assert value.isascii()
        widths=[84]+[30]*len(value)
        values=[f'{len(value.encode("utf-8"))}usize']+list(value)
        total=sum(widths)
        d.text(x+total/2,y-18,name,'code heading',color='#64748b' if muted else BLUE)
        d.parts.append(f'<rect x="{x}" y="{y}" width="{total}" height="36" rx="6" fill="{"#f1f5f9" if muted else "#ffffff"}" stroke="#cbd5e1"/>')
        offset=0
        for width,value in zip(widths,values):
            if offset:d.parts.append(f'<path d="M{x+offset},{y} v36" class="grid"/>')
            d.text(x+offset+width/2,y+18,value,'code',color='#64748b' if muted else INK)
            offset+=width
        return x,y+18
    def pin(d,x,y):
        d.text(x+32,y-18,'Pin','heading')
        d.parts.append(f'<rect x="{x}" y="{y}" width="64" height="40" rx="6" fill="#fff" stroke="#cbd5e1"/>')
        return x+32,y+20
    d=Diagram(640,240,'Pin arkasındaki merhaba metni','Pin, 7 baytlık merhaba metnini tutan String değerine işaret eder. Kesikli çerçeve, String türünün Unpin uyguladığını gösterir.')
    panel(d,200,40,404,164)
    end=string_row(d,232,126,'metin','merhaba')
    start=pin(d,32,124);d.arrow(start,end)
    d.save(8,chapter=17)
    d=Diagram(840,350,'String değerini merhaba yerine selam ile değiştirme','Önceki metin1 değeri merhaba 7 bayttır ve soluk gösterilir. Pin artık metin2 adıyla gösterilen 5 baytlık selam değerine işaret eder. String, Unpin uyguladığı için bu değişim güvenlidir.')
    panel(d,410,150,394,166)
    string_row(d,32,245,'metin1','merhaba',muted=True)
    end=string_row(d,444,244,'metin2','selam')
    start=pin(d,72,65);d.arrow(start,end)
    d.save(9,chapter=17)

if __name__ == '__main__':
    main()
    pin_diagrams()

<script lang="ts">
  import { reportState } from '../lib/reportState.svelte';
  import { PDFDocument, StandardFonts, rgb } from 'pdf-lib';

  const today = new Date().toLocaleDateString('es-CO', { year: 'numeric', month: 'long', day: 'numeric' });

  async function downloadPDF() {
    const pdfDoc = await PDFDocument.create();
    const timesRomanFont = await pdfDoc.embedFont(StandardFonts.TimesRoman);
    const timesBoldFont = await pdfDoc.embedFont(StandardFonts.TimesRomanBold);

    const page = pdfDoc.addPage([612, 792]); // Letter size
    const { width, height } = page.getSize();
    const fontSize = 11;
    let currentY = height - 100;

    const drawLine = (text: string, font = timesRomanFont, size = fontSize, x = 70) => {
        page.drawText(text, { x, y: currentY, size, font, color: rgb(0, 0, 0) });
        currentY -= size + 10;
    };

    // Header
    page.drawText('PROCURADURÍA GENERAL DE LA NACIÓN', {
        x: width / 2 - 120,
        y: currentY,
        size: 10,
        font: timesBoldFont,
        color: rgb(0.4, 0.4, 0.4)
    });
    currentY -= 40;

    drawLine(`Bogotá D.C., ${today}`);
    currentY -= 20;

    drawLine(`ASUNTO: DENUNCIA POR PRESUNTAS IRREGULARIDADES EN EL CONTRATO No. ${reportState.contractId || 'N/A'}`, timesBoldFont);
    currentY -= 20;

    drawLine('Respetados Señores,');
    currentY -= 10;

    const bodyText = `Yo, ciudadano en ejercicio de mis derechos de control social, presento ante su despacho denuncia formal por ${reportState.irregularityType === 'Seleccione una opción...' ? 'irregularidades contractuales' : reportState.irregularityType} detectadas en los procesos de contratación de la entidad ${reportState.entity}.`;

    // Simple text wrapping for demo (real impl would be more robust)
    const words = bodyText.split(' ');
    let line = '';
    for (const word of words) {
        if (line.length + word.length > 80) {
            drawLine(line);
            line = word + ' ';
        } else {
            line += word + ' ';
        }
    }
    drawLine(line);

    if (reportState.description) {
        currentY -= 10;
        drawLine('DESCRIPCIÓN DE LOS HECHOS:', timesBoldFont);
        const descWords = reportState.description.split(' ');
        let descLine = '';
        for (const word of descWords) {
            if (descLine.length + word.length > 80) {
                drawLine(descLine, timesRomanFont, fontSize, 90);
                descLine = word + ' ';
            } else {
                descLine += word + ' ';
            }
        }
        drawLine(descLine, timesRomanFont, fontSize, 90);
    }

    currentY -= 40;
    drawLine('Atentamente,');
    currentY -= 30;
    drawLine('CONTROL CIUDADANO - VEEDURIA IA', timesBoldFont);

    // Save to history
    saveToHistory();

    const pdfBytes = await pdfDoc.save();
    const blob = new Blob([pdfBytes], { type: 'application/pdf' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `denuncia_${reportState.contractId || 'contrato'}.pdf`;
    link.click();
  }

  function sendEmail() {
    saveToHistory();
    const subject = `Denuncia Irregularidades Contrato ${reportState.contractId}`;
    const body = `Respetados Señores Procuraduría,\n\nPresento denuncia por ${reportState.irregularityType} en el contrato ${reportState.contractId}.\n\nDescripción:\n${reportState.description}\n\nAtentamente,\nCiudadano Veedor`;
    window.location.href = `mailto:quejas@procuraduria.gov.co?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`;
  }

  function saveToHistory() {
    const history = JSON.parse(localStorage.getItem('veeduria_reports') || '[]');
    const newReport = {
      id: reportState.contractId,
      entity: reportState.entity,
      type: reportState.irregularityType,
      date: new Date().toISOString()
    };
    // Avoid duplicates
    if (!history.some((h: any) => h.id === newReport.id && h.type === newReport.type)) {
      history.unshift(newReport);
      localStorage.setItem('veeduria_reports', JSON.stringify(history.slice(0, 10))); // Keep last 10
    }
  }

  function copyToClipboard() {
    const text = `DENUNCIA FORMAL ANTE LA PROCURADURÍA
Fecha: ${today}
Contrato: ${reportState.contractId}
Tipo: ${reportState.irregularityType}
Descripción: ${reportState.description}
Generado por Veeduría IA`;
    navigator.clipboard.writeText(text);
    alert('Texto copiado al portapapeles');
  }
</script>

<div class="glass-panel p-8 rounded-2xl bg-white/[0.02] flex flex-col h-full">
  <h2 class="text-2xl font-bold text-gray-300 mb-6 flex items-center gap-3">
    <span class="flex items-center justify-center w-8 h-8 rounded-full bg-brand-500 text-white text-sm">2</span>
    Vista Previa
  </h2>

  <!-- Document Preview -->
  <div class="bg-white text-black p-8 rounded-lg shadow-2xl origin-top mb-8 flex-1 overflow-y-auto min-h-[400px] border border-gray-200">
    <div class="text-center mb-8 border-b-2 border-gray-100 pb-4">
      <div class="w-16 h-16 mx-auto bg-brand-50 rounded-full flex items-center justify-center mb-2">
        <span class="text-brand-500 font-bold">V-IA</span>
      </div>
      <h3 class="font-serif font-bold uppercase text-[10px] tracking-widest text-gray-600">Procuraduría General de la Nación</h3>
      <p class="text-[8px] text-gray-400 mt-1">SISTEMA CIUDADANO DE CONTROL FISCAL</p>
    </div>

    <div class="space-y-6 text-[11px] text-justify leading-relaxed font-serif text-gray-800">
      <div class="flex justify-between font-bold">
        <span>Bogotá D.C., {today}</span>
      </div>

      <p>
        <strong>ASUNTO:</strong> DENUNCIA POR PRESUNTAS IRREGULARIDADES EN EL CONTRATO No.
        <span class="bg-brand-50 px-1 border-b border-brand-200">{reportState.contractId || '________________'}</span>
      </p>

      <p>Respetados Señores,</p>

      <p>
        Yo, ciudadano en ejercicio de mis derechos de control social, presento ante su despacho denuncia formal por
        <strong>{reportState.irregularityType === 'Seleccione una opción...' ? 'irregularidades contractuales' : reportState.irregularityType}</strong>
        detectadas en los procesos de contratación de la entidad <strong>{reportState.entity}</strong>.
      </p>

      {#if reportState.description}
        <div class="bg-gray-50 p-3 border-l-2 border-brand-500 italic">
          {reportState.description}
        </div>
      {/if}

      <p>
        Solicito se inicien las investigaciones correspondientes para salvaguardar el patrimonio público y garantizar los principios de la contratación estatal.
      </p>

      <div class="mt-12 pt-8 border-t border-gray-100">
        <p>Atentamente,</p>
        <div class="h-10"></div>
        <p class="font-bold">CONTROL CIUDADANO - VEEDURIA IA</p>
        <p class="text-[8px] text-gray-400 italic">Documento generado mediante análisis de datos abiertos.</p>
      </div>
    </div>
  </div>

  <div class="flex gap-4">
    <button
      onclick={downloadPDF}
      class="flex-1 px-6 py-3 rounded-xl bg-brand-500 text-white font-medium hover:bg-brand-400 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
      disabled={!reportState.contractId}
    >
      Descargar PDF
    </button>
    <button
      onclick={copyToClipboard}
      class="flex-1 px-6 py-3 rounded-xl glass-panel text-white hover:bg-white/10 transition-all border border-white/20 disabled:opacity-50"
      disabled={!reportState.contractId}
    >
      Copiar Texto
    </button>
    <button
      onclick={sendEmail}
      class="flex-1 px-6 py-3 rounded-xl glass-panel text-white hover:bg-white/10 transition-all border border-white/20 disabled:opacity-50 disabled:cursor-not-allowed"
      disabled={!reportState.contractId}
    >
      Enviar Email
    </button>
  </div>
</div>

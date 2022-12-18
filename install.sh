cargo install crust_ide

echo "[Desktop Entry]
Version=1.0
Name=Crust
GenericName[ca]=Editor de text
GenericName[de]=Texteditor
GenericName[eo]=Tekstoredaktilo
GenericName[es]=Editor de texto
GenericName[fi]=Tekstinmuokkain
GenericName[fr]=Éditeur de texte
GenericName[ga]=Eagarthóir Téacs
GenericName[it]=Editor di testi
GenericName[ja]=テキストエディタ
GenericName[ru]=Текстовый редактор
GenericName[sr]=Едитор текст
GenericName[tr]=Metin Düzenleyici
GenericName[uk]=Редактор Тексту
GenericName[zh_CN]=文本编辑器
GenericName=Text Editor
# Translators: This is the comment used in the Vim desktop file
Comment[ca]=Edita fitxers de text
Comment[de]=Textdateien bearbeiten
Comment[eo]=Redakti tekstajn dosierojn
Comment[es]=Editar archivos de texto
Comment[fi]=Muokkaa tekstitiedostoja
Comment[fr]=Éditer des fichiers texte
Comment[ga]=Cuir comhaid téacs in eagar
Comment[it]=Edita file di testo
Comment[ja]=テキストファイルを編集します
Comment[ru]=Редактирование текстовых файлов
Comment[sr]=Уређујте текст фајлове
Comment[tr]=Metin dosyaları düzenleyin
Comment[uk]=Редагувати текстові файли
Comment[zh_CN]=编辑文本文件
Comment=Edit text files
# The translations should come from the po file. Leave them here for now, they will
# be overwritten by the po file when generating the desktop.file.
GenericName[da]=Teksteditor
GenericName[pl]=Edytor tekstu
GenericName[is]=Ritvinnsluforrit
Comment[af]=Redigeer tekslêers
Comment[am]=የጽሑፍ ፋይሎች ያስተካክሉ
Comment[ar]=حرّر ملفات نصية
Comment[az]=Mətn fayllarını redaktə edin
Comment[be]=Рэдагаваньне тэкставых файлаў
Comment[bg]=Редактиране на текстови файлове
Comment[bn]=টেক্স্ট ফাইল এডিট করুন
Comment[bs]=Izmijeni tekstualne datoteke
Comment[cs]=Úprava textových souborů
Comment[cy]=Golygu ffeiliau testun
Comment[da]=Rediger tekstfiler
Comment[el]=Επεξεργασία αρχείων κειμένου
Comment[en_CA]=Edit text files
Comment[en_GB]=Edit text files
Comment[et]=Redigeeri tekstifaile
Comment[eu]=Editatu testu-fitxategiak
Comment[fa]=ویرایش پرونده‌های متنی
Comment[gu]=લખાણ ફાઇલોમાં ફેરફાર કરો
Comment[he]=ערוך קבצי טקסט
Comment[hi]=पाठ फ़ाइलें संपादित करें
Comment[hr]=Uređivanje tekstualne datoteke
Comment[hu]=Szövegfájlok szerkesztése
Comment[id]=Edit file teks
Comment[is]=Vinna með textaskrár
Comment[kn]=ಪಠ್ಯ ಕಡತಗಳನ್ನು ಸಂಪಾದಿಸು
Comment[ko]=텍스트 파일을 편집합니다
Comment[lt]=Redaguoti tekstines bylas
Comment[lv]=Rediģēt teksta failus
Comment[mk]=Уреди текстуални фајлови
Comment[ml]=വാചക രചനകള് തിരുത്തുക
Comment[mn]=Текст файл боловсруулах
Comment[mr]=गद्य फाइल संपादित करा
Comment[ms]=Edit fail teks
Comment[nb]=Rediger tekstfiler
Comment[ne]=पाठ फाइललाई संशोधन गर्नुहोस्
Comment[nl]=Tekstbestanden bewerken
Comment[nn]=Rediger tekstfiler
Comment[no]=Rediger tekstfiler
Comment[or]=ପାଠ୍ଯ ଫାଇଲଗୁଡ଼ିକୁ ସମ୍ପାଦନ କରନ୍ତୁ
Comment[pa]=ਪਾਠ ਫਾਇਲਾਂ ਸੰਪਾਦਨ
Comment[pl]=Edytuj pliki tekstowe
Comment[pt]=Editar ficheiros de texto
Comment[pt_BR]=Edite arquivos de texto
Comment[ro]=Editare fişiere text
Comment[sk]=Úprava textových súborov
Comment[sl]=Urejanje datotek z besedili
Comment[sq]=Përpuno files teksti
Comment[sr@Latn]=Izmeni tekstualne datoteke
Comment[sv]=Redigera textfiler
Comment[ta]=உரை கோப்புகளை தொகுக்கவும்
Comment[th]=แก้ไขแฟ้มข้อความ
Comment[tk]=Metin faýllary editle
Comment[vi]=Soạn thảo tập tin văn bản
Comment[wa]=Asspougnî des fitchîs tecses
Comment[zh_TW]=編輯文字檔
TryExec=vim
Exec=vim %F
Terminal=true
Type=Application
# Translators: Search terms to find this application. Do NOT change the semicolons! The list MUST also end with a semicolon!
Keywords[ca]=Text;editor;
Keywords[de]=Text;Editor;
Keywords[eo]=Teksto;redaktilo;
Keywords[es]=Texto;editor;
Keywords[fi]=Teksti;muokkain;editori;
Keywords[fr]=Texte;éditeur;
Keywords[ga]=Téacs;eagarthóir;
Keywords[it]=Testo;editor;
Keywords[ja]=テキスト;エディタ;
Keywords[ru]=текст;текстовый редактор;
Keywords[sr]=Текст;едитор;
Keywords[tr]=Metin;düzenleyici;
Keywords[uk]=текст;редактор;
Keywords[zh_CN]=Text;editor;文本;编辑器;
Keywords=Text;editor;
Comment=Extremely fast text editor
Exec=~/.cargo/bin/crust
Icon=`pwd`/crust.png
Terminal=false
StartupWMClass=crust
Type=Application
Categories=Utility;TextEditor;Utility;
MimeType=text/english;text/plain;text/x-makefile;text/x-c++hdr;text/x-c++src;text/x-chdr;text/x-csrc;text/x-java;text/x-moc;text/x-pascal;text/x-tcl;text/x-tex;application/x-shellscript;text/x-c;text/x-c++;
" >> crust.desktop

sudo cp crust.desktop /usr/share/applications/

cd ~/.config
sudo mkdir crust
cd crust
sudo touch crust.ui
sudo touch crust.css


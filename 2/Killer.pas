{ Naprogramoval J. Pikl / Jack Pkiller }
{ Roidsoft company }

program Killer_;
uses crt;

var
  XM, YM, XM2, YM2, XE, YE, XE1, YE1, XE2, YE2, XE3, YE3, X, Y: integer;
  rychlost, time:integer;
  volba: char;
  name:string;
  Matice: array [1..80, 1..50] of Integer;      { Mriz obrazovka  }

begin
  Clrscr;
  TextMode(C80 + Font8x8);      { Obrazovka 80 x 50 }
  Gotoxy(15, 10);
  Textcolor(7);
  Write('Vitejte ve hre ');
  Textcolor(4);
  Writeln('Zabijak');
  Gotoxy(15, 25);
  Textcolor(7);
  Write('Zadej tve jmeno: ');
  Readln(name);
  Gotoxy(8, 31);
  Write('(!!! Cim vyssi cislo rychlosti tim je i hra rychlejsi !!!)');
  Gotoxy(15, 28);
  Textcolor(7);
  Write('Zadej rychlost (1 - 5): ');
  Readln(rychlost);
  Clrscr;
  Gotoxy(10, 1);
  Textcolor(7);
  Write('Jmeno: ',name);

  XM := 8;
  YM := 7;
  XM2 := 8;
  YM2 := 37;
  XE := 40;             { Nepritel }
  YE := 20;
  XE1 := 40;             { Nepritel 1 }
  YE1 := 40;
  XE2 := 60;             { Nepritel 2 }
  YE2 := 47;
  XE3 := 10;             { Nepritel 2 }
  YE3 := 47;
  Y := 8;
  time := 0;

  Gotoxy(1, Y);
  begin
   Textcolor(4);
   Writeln('Z');
   Writeln;
   Writeln('a');
   Writeln;
   Writeln('b');
   Writeln;
   Writeln('i');
   Writeln;
   Writeln('j');
   Writeln;
   Writeln('a');
   Writeln;
   Writeln('K');
  end;

  Textcolor(7);

  { Naplnime matici nulami }
  for X:= 2 to 80 do
    for Y:= 1 to 50 do
      Matice[X, Y] := 0;
  { Cil }
  Textcolor(7);
  Gotoxy(XM2, YM2);
  Writeln('Cil');

  { Dolni a horni okraj }
  for X:= 2 to 80 do
    begin
      Matice[X, 2]:= 1;
      Gotoxy(X,2);
      Textcolor(1);
      Write(#219);
      Matice[X, 48]:= 1;
      Gotoxy(X,48);
      Write(#219);
    end;
  { Levy a pravy okraj }
  for Y:= 3 to 47 do
    begin
      Matice[3, Y]:= 1;
      Gotoxy(3, Y);
      Textcolor(1);
      Write(#219);
      Matice[79, Y]:= 1;
      Gotoxy(79, Y);
      Write(#219);
      Textcolor(6);
      Matice[80, Y]:= 1;
      Gotoxy(80, Y);
      Write(#219);
    end;
  { Levy status }
  for Y:= 3 to 47 do
    begin
      Matice[2, Y]:= 1;
      Gotoxy(2, Y);
      Textcolor(6);
      Write(#219);
    end;
  Readln;

  repeat
    Gotoxy(26, 1);
    begin
      Textcolor(7);
      Write('Rychlost: ');
      Textcolor(3);
      Write(rychlost);
    end;

    Gotoxy(40, 1);
    begin
      time := time + 1;
      Textcolor(7);
      Write('Cas: ');
      Textcolor(3);
      Write(time,' / 500');
    end;
    Gotoxy(55, 1);
    begin
      Textcolor(7);
      Write('Souradnice ');
      Textcolor(3);
      Write('X:', XM,' ');
      Gotoxy(72, 1);
      Write('Y:', YM,' ');
    end;

      { Cil }
    Textcolor(7);
    Gotoxy(XM2, YM2);
    Writeln('Cil');

      { Line 1 }
    for X:= 3 to 55 do
      begin
        Matice[X, 26]:= 1;
        Gotoxy(X, 26);
        Textcolor(6);
        Write(#219);
      end;
      { Line 2 }
    for Y:= 15 to 26 do
      begin
        Matice[35, Y]:= 1;
        Gotoxy(35, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 3 }
    for X:= 3 to 15 do
      begin
        Matice[X, 20]:= 1;
        Gotoxy(X, 20);
        Textcolor(6);
        Write(#219);
      end;
    { Line 4 }
    for Y:= 3 to 15 do
      begin
        Matice[15, Y]:= 1;
        Gotoxy(15, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 5 }
    for X:= 21 to 30 do
      begin
        Matice[X, 16]:= 1;
        Gotoxy(X, 16);
        Textcolor(6);
        Write(#219);
      end;
    { Line 6 }
    for Y:= 6 to 20 do
      begin
        Matice[21, Y]:= 1;
        Gotoxy(21, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 7 }
    for Y:= 24 to 25 do
      begin
        Matice[11, Y]:= 1;
        Gotoxy(11, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 8 }
    for X:= 10 to 15 do
      begin
        Matice[X, 12]:= 1;
        Gotoxy(X, 12);
        Textcolor(6);
        Write(#219);
      end;
    { Line 9 }
    for Y:= 4 to 10 do
      begin
        Matice[41, Y]:= 1;
        Gotoxy(41, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 10 }
    for Y:= 12 to 20 do
      begin
        Matice[41, Y]:= 1;
        Gotoxy(41, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 11 }
    for X:= 45 to 55 do
      begin
        Matice[X, 12]:= 1;
        Gotoxy(X, 12);
        Textcolor(6);
        Write(#219);
      end;
    { Line 12 }
    for X:= 59 to 74 do
      begin
        Matice[X, 18]:= 1;
        Gotoxy(X, 18);
        Textcolor(6);
        Write(#219);
      end;
    { Line 13 }
    for Y:= 4 to 15 do
      begin
        Matice[65, Y]:= 1;
        Gotoxy(65, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 14 }
    for Y:= 10 to 17 do
      begin
        Matice[73, Y]:= 1;
        Gotoxy(73, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 15 }
    for X:= 62 to 70 do
      begin
        Matice[X, 24]:= 1;
        Gotoxy(X, 24);
        Textcolor(6);
        Write(#219);
      end;
    { Line 16 }
    for Y:= 30 to 42 do
      begin
        Matice[15, Y]:= 1;
        Gotoxy(15, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 17 }
    for X:= 3 to 6 do
      begin
        Matice[X, 32]:= 1;
        Gotoxy(X, 32);
        Textcolor(6);
        Write(#219);
      end;
    { Line 18 }
    for X:= 12 to 15 do
      begin
        Matice[X, 32]:= 1;
        Gotoxy(X, 32);
        Textcolor(6);
        Write(#219);
      end;
    { Line 19 }
    for Y:= 26 to 36 do
      begin
        Matice[29, Y]:= 1;
        Gotoxy(29, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 20 }
    for Y:= 30 to 47 do
      begin
        Matice[35, Y]:= 1;
        Gotoxy(35, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 21 }
    for Y:= 33 to 45 do
      begin
        Matice[45, Y]:= 1;
        Gotoxy(45, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 22 }
    for Y:= 27 to 45 do
      begin
        Matice[55, Y]:= 1;
        Gotoxy(55, Y);
        Textcolor(6);
        Write(#219);
      end;
    { Line 23 }
    for X:= 55 to 73 do
      begin
        Matice[X, 28]:= 1;
        Gotoxy(X, 28);
        Textcolor(6);
        Write(#219);
      end;
    { Line 24 }
    for X:= 60 to 79 do
      begin
        Matice[X, 32]:= 1;
        Gotoxy(X, 32);
        Textcolor(6);
        Write(#219);
      end;
    { Line 25 }
    for X:= 55 to 72 do
      begin
        Matice[X, 36]:= 1;
        Gotoxy(X, 36);
        Textcolor(6);
        Write(#219);
      end;
    { Line 26 }
    for X:= 40 to 45 do
      begin
        Matice[X, 36]:= 1;
        Gotoxy(X, 36);
        Textcolor(6);
        Write(#219);
      end;
    { Line 27 }
    for X:= 45 to 48 do
      begin
        Matice[X, 40]:= 1;
        Gotoxy(X, 40);
        Textcolor(6);
        Write(#219);
      end;
    { Line 28 }
    for X:= 23 to 33 do
      begin
        Matice[X, 40]:= 1;
        Gotoxy(X, 40);
        Textcolor(6);
        Write(#219);
      end;
    { Line 29 }
    for X:= 27 to 35 do
      begin
        Matice[X, 10]:= 1;
        Gotoxy(X, 10);
        Textcolor(6);
        Write(#219);
      end;
    { Line 30 }
    for X:= 3 to 10 do
      begin
        Matice[X, 40]:= 1;
        Gotoxy(X, 40);
        Textcolor(6);
        Write(#219);
      end;
    { Line 31 }
    for Y:= 40 to 47 do
      begin
        Matice[19, Y]:= 1;
        Gotoxy(19, Y);
        Textcolor(6);
        Write(#219);
      end;

    if KeyPressed then          { Je-li zmacknuta klavesa ... }
    begin
      Volba := Readkey;         { Zjisti kod klavesy }
      if Volba = #0 then        { Kdyz je to ridici klavesa ...}
        Volba := Readkey;       { ... tak precti jeji kod }
    end;

    if (volba = #72) and (Matice[XM, YM - 1] = 0) then  { Sipka nahoru }
      YM:= YM - 2;
    if (volba = #75) and (Matice[XM - 1, YM] = 0) then  { Sipka doleva }
      XM:= XM - 2;
    if (volba = #80) and (Matice[XM, YM + 1] = 0) then  { Sipka dolu }
      YM:= YM + 2;
    if (volba = #77) and (Matice[XM + 1, YM] = 0) then  { Sipka doprava }
      XM:= XM + 2;

    Gotoxy(XM, YM);             { Zobrazim hrace }
    Textcolor(3);
    Write(#1);
    Gotoxy(XE, YE);             { Zobrazim nepritele1 }
    Textcolor(4);
    Write('@');
    Gotoxy(XE1, YE1);             { Zobrazim nepritele1 }
    Textcolor(4);
    Write('@');
    Gotoxy(XE2, YE2);             { Zobrazim nepritele1 }
    Textcolor(4);
    Write('@');
    Gotoxy(XE3, YE3);             { Zobrazim nepritele1 }
    Textcolor(4);
    Write('@');
      if rychlost = 1 then
        Delay(100);
      if rychlost = 2 then
        Delay(80);
      if rychlost = 3 then
        Delay(60);
      if rychlost = 4 then
        Delay(40);
      if rychlost = 5 then
        Delay(20)
      else
        Delay(50);
    Gotoxy(XM, YM);             { A budeme mazat }
    Write(' ');
    Gotoxy(XE, YE);             { A budeme mazat }
    Write(' ');
    Gotoxy(XE1, YE1);             { A budeme mazat }
    Write(' ');
    Gotoxy(XE2, YE2);             { A budeme mazat }
    Write(' ');
    Gotoxy(XE3, YE3);             { A budeme mazat }
    Write(' ');

 { Nepritel jde primo na nas }
     if XE <> XM then
       begin
         if XE < XM then               { Nepritel pujde doprava }
         begin
           if Matice[XE + 1, YE] = 0 then    { 0 - Neni tam prekazka }
           XE := XE + 1;
         end
       else                             { Nepritel pujde doleva  }
       begin
         if Matice[XE - 1, YE] = 0 then        { 0 - Neni tam prekazka }
           XE := XE - 1
       end;
      end;
       if YE <> YM then
       begin
         if YE < YM then                       { Nepritel jde dolu }
         begin
           if Matice[XE, YE + 1] = 0 then    { 0 - Neni tam prekazka }
           YE := YE + 1;
         end
       else
         begin
           if Matice[XE, YE - 1] = 0 then    { 0 - Neni tam prekazka }
             YE := YE - 1;
         end;
      end;
 { Nepritel 1 jde primo na nas }
     if XE1 <> XM then
       begin
         if XE1 < XM then               { Nepritel pujde doprava }
         begin
           if Matice[XE1 + 1, YE1] = 0 then    { 0 - Neni tam prekazka }
           XE1 := XE1 + 1;
         end
       else                             { Nepritel pujde doleva  }
       begin
         if Matice[XE1 - 1, YE1] = 0 then        { 0 - Neni tam prekazka }
           XE1 := XE1 - 1
       end;
      end;
       if YE1 <> YM then
       begin
         if YE1 < YM then                       { Nepritel jde dolu }
         begin
           if Matice[XE1, YE1 + 1] = 0 then    { 0 - Neni tam prekazka }
           YE1 := YE1+ 1;
         end
       else
         begin
           if Matice[XE1, YE1 - 1] = 0 then    { 0 - Neni tam prekazka }
             YE1 := YE1 - 1;
         end;
      end;
 { Nepritel 2 jde primo na nas }
     if XE2 <> XM then
       begin
         if XE2 < XM then               { Nepritel pujde doprava }
         begin
           if Matice[XE2 + 1, YE2] = 0 then    { 0 - Neni tam prekazka }
           XE2 := XE2 + 1;
         end
       else                             { Nepritel pujde doleva  }
       begin
         if Matice[XE2 - 1, YE2] = 0 then        { 0 - Neni tam prekazka }
           XE2 := XE2 - 1
       end;
      end;
       if YE2 <> YM then
       begin
         if YE2 < YM then                       { Nepritel jde dolu }
         begin
           if Matice[XE2, YE2 + 1] = 0 then    { 0 - Neni tam prekazka }
           YE2 := YE2+ 1;
         end
       else
         begin
           if Matice[XE2, YE2 - 1] = 0 then    { 0 - Neni tam prekazka }
             YE2 := YE2 - 1;
         end;
      end;
 { Nepritel 3 jde primo na nas }
     if XE3 <> XM then
       begin
         if XE3 < XM then               { Nepritel pujde doprava }
         begin
           if Matice[XE3 + 1, YE3] = 0 then    { 0 - Neni tam prekazka }
           XE3 := XE3 + 1;
         end
       else                             { Nepritel pujde doleva  }
       begin
         if Matice[XE3 - 1, YE3] = 0 then        { 0 - Neni tam prekazka }
           XE3 := XE3 - 1
       end;
      end;
       if YE3 <> YM then
       begin
         if YE3 < YM then                       { Nepritel jde dolu }
         begin
           if Matice[XE3, YE3 + 1] = 0 then    { 0 - Neni tam prekazka }
           YE3 := YE3+ 1;
         end
       else
         begin
           if Matice[XE3, YE3 - 1] = 0 then    { 0 - Neni tam prekazka }
             YE3 := YE3 - 1;
         end;
      end;

    { ----- Nesrazenise souperu ----- }
    if (XE = XE1) and (YE = YE1) then
      XE1 := XE1 + 1;
    if (XE = XE2) and (YE = YE2) then
      XE2 := XE2 + 1;
    if (XE1 = XE2) and (YE1 = YE2) then
      XE2 := XE2 + 1;
    if (XE3 = XE1) and (YE3 = YE1) then
      XE3 := XE3 + 1;
    if (XE3 = XE2) and (YE3 = YE2) then
      XE3 := XE3 + 1;
    if (XE3 = XE) and (YE3 = YE) then
      XE3 := XE3 + 1;
    { ----- Nesrazenise souperu ----- }

  until (Volba = #27) or (XM = XM2) and (YM = YM2) or (XM = XE) and (YM = YE) or (XM = XE1) and (YM = YE1) or (XM = XE2) and
  (YM = YE2) or (XM = XE3) and (YM = YE3) or (time = 500);
  if (XM = XE) and (YM = YE) or (XM = XE1) and (YM = YE1) or (XM = XE2) and (YM = YE2) or (XM = XE3) and (YM = YE3) then
   begin
     Clrscr;
     Gotoxy(25, 20);
     Textcolor(7);
     Write('!!! Hrac ');
     Textcolor(6);
     Write(name);
     Textcolor(7);
     Write(' porazen !!!');
   end;
  if time = 0 then
   begin
     Clrscr;
     Gotoxy(25, 20);
     Writeln('!!! Herni cas vyprsel !!!');
   end;
  if (XM = XM2) and (YM = YM2) then
    begin
      Clrscr;
      Gotoxy(20, 20);
      Writeln('!!! Gratuluji, prosel jsi 1. kolo !!!');
    end;
  Delay(1500);
  Clrscr;
  Textcolor(7);
  Gotoxy(25, 20);
  Writeln('Doufam, ze jste se dobre pobavili...');
  Gotoxy(7, 22);
  Writeln('Pripadne dotazy ci jine informace piste na adresu: rapmaker@seznam.cz');
  Gotoxy(27, 24);
  Writeln('Naprogramoval: Jan Pikl');
  Readln;
  TextMode(C80);      { Obrazovka 80 x 25 }
end.
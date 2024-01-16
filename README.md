# SAT solver (za alokaciju registara)

Projekt iz predmeta [Paralelizam i konkurentnost](https://www.fer.unizg.hr/predmet/pik) na temu ubrzanja (paraleliziranja) SAT solvera.  

## Općenito o projektu
Rad je pisan u programskom jeziku Rust, te za paralelizaciju koristi "crate" Reyon. Za testiranje programa na skupu primjera napisana je bash skripta. Za pokretanje iste potrebna je unix mašina (nije moguće pokrenuti u Windowsu, osim preko WSL-a, Git basha ili sličnih stvari). Kako bi se pokrenula potrebno je izvršiti komandu `./test <ime_testnog_foldera>` dok se nalazimo u glavnom direktoriju programa.  

#### Troubleshoot skripte
**Napomena** ako se repozitorij skine na Windowsu, postoji mogućnost da će Windows automatski promijeniti znakove novog reda u skripti te ona više neće raditi (dobit će se error `bash bad interpreter`), kako bi se to izbjeglo repozitorij je potrebno skinuti odmah na unix sistemu ili skinuti program poput `dos2unix` koji će te znakove prebaciti nazad.  
**Napomena 2** ako se skripta ne želi izvršiti pokrenuti komandu `sudo chmod u+x <skripta>`

## Testni primjeri
Skup testnih primjera se nalaze u `/tests` folderu gdje se mogu i ručno dodati razni testovi. Testovi su preuzeti s [ove](https://www.cs.ubc.ca/~hoos/SATLIB/benchm.html) stranice, konkretno `CBS_k3_n100_m449_b90` skup. Ako se želi promijeniti testni skup potrebno je skinuti neki od primjere sa stranice (ili napisati svoje) i radi lakšeg crtanja grafa potrebno je promijeniti ime primjera u `redni_broj.cnf`, sljedeće se može napraviti pomoću komande `(paste <(ls)  <(ls | nl | cut -f1) | xargs -n 1 echo | xargs -n 2 mv) && ls | xargs -I xx mv xx xx.cnf` u testnom folderu. Također je u skripti moguće promijeniti broj dretvi za koje želimo izvršiti test, u liniji `threads=(1 2 4 8 12 16 20 32)` dodati broj koji želimo (ili maknuti one koje ne želimo). 

## Rezultati 
Nakon izvršavanja skripte rezultati će biti zapisani u datoteku `results.txt` gdje prvi red mora biti format ispisa radi crtanja grafa (`serialOrParallel,thread,duration,example`), a sljedeći redovi prikazuju rezultate konkretnog primjera u milisekundama. 

#### Crtanje grafa
Ako želimo prikazati graf postoji R skripta koja se nalazi u `doc/results_plot.Rmd` koju je potrebno otvoriti u R studiju, preimenovati `results.txt` u `results.csv` i pokrenuti ju, evenutalno promijeniti "breakove" u ovoj liniji `scale_y_continuous(breaks = c(4, 6.97, 8, 12, 16))` radi boljeg izgleda grafa.
 
## Dokumentacija 
Dokumentacija je pisana u Latexu pomoću IEEE predloška te se nalazi u direktoriju `doc/par_sat_solver.pdf`.

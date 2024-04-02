
use std::{cmp::Ordering,  io};
#[derive( Clone)]
struct  Equipos{

nombre:String,
pj:u8,
pg:u8,
pe:u8,
pp:u8,
gf:u8,
ge:u8,
puntos:u8,
grupo:String,


}

impl  Equipos {

fn mostrar(&self){

    println!("{}  {}  {}  {}  {}  {}  {}  {}",self.nombre,self.pj,self.pg,self.pe,self.pp,self.gf,self.ge,self.puntos)

}
    
}

struct Partido{

equipo1:String,
equipo2:String,
gol1:u8,
gol2:u8,


}



struct menu{

    
}


impl menu {
    
   fn cargar(&self,equipo:&mut Vec<Equipos>,grupo_a:&mut Vec<Partido>,grupo_b:&mut Vec<Partido>){

     equipo.clear();

     grupo_a.clear();

     grupo_b.clear();
     
     print!("{}[2J", 27 as char);

      println!("El numero de equipos debe ser mayor a 6 y par");
      
      println!();

      let mut grup=String::new();

      let mut cont=1;
      
      loop {

       println!();
       
       println!("Ingrese el nombre del equipo");

       let mut nom=String::new();

       io::stdin().read_line(& mut nom).expect("fallo la lectura");
  
       if cont%2==0{

          grup="A".to_string();
           
       }else{

          grup="B".to_string();          

       }   

       cont+=1; 

       equipo.push(Equipos{nombre:nom.trim().to_string(),pj:0,pg:0,pe:0,pp:0,gf:0,ge:0,puntos:0,grupo:grup});

          if equipo.len()>=6 && equipo.len()%2==0{

            println!(); 

            println!("Desea ingresar mas equipos si o no");

            let mut res=String::new();
            
            io::stdin().read_line(& mut res).expect("fallo la lectura");

            if res.trim()!="si"{

                break;

            }
       }

      }
      
      let mut A:Vec<&String>=vec![];

      let mut B:Vec<&String>=vec![];


      for i in 0..equipo.len() {
          
      if &equipo[i].grupo=="A" {

         A.push(&equipo[i].nombre)
          
      }else{

         B.push(&equipo[i].nombre)

      } 
    }

      for i in 0..equipo.len()/2{

        for j in i+1..equipo.len()/2{

           grupo_a.push(Partido{equipo1:A[i].to_string(),equipo2:A[j].to_string(),gol1:0,gol2:0});

           grupo_b.push(Partido{equipo1:B[i].to_string(),equipo2:B[j].to_string(),gol1:0,gol2:0});
              
        }
      
      }

      print!("{}[2J", 27 as char);

    self.opciones(equipo, grupo_a, grupo_b)  ;  
 
}

   fn ingresar_resultados(&self,equipo:&mut Vec<Equipos>,grupo_a:&mut Vec<Partido>,grupo_b:&mut Vec<Partido>){

    print!("{}[2J", 27 as char);

     println!("Ingresa los resuldos del grupo A y B");

     println!();
     
     println!("Grupo A");

     for i in 0..grupo_a.len(){

        let mut gols1=String::new();

        let  mut gols2=String::new();

        println!();

        println!("{} vs {}",grupo_a[i].equipo1,grupo_a[i].equipo2);
        
       loop{

        println!("Goles del equipo {}",grupo_a[i].equipo1);

        io::stdin().read_line(&mut gols1).expect("fallo la lectura");
        
        let mut gols1=gols1.trim().to_string();

        if gols1.parse::<u8>().is_ok(){

        }else {
            continue;
        }

        let gols1:u8=gols1.parse().expect("fallo");

        grupo_a[i].gol1=gols1;

        break;

        }

        loop{

            println!("Goles del equipo {}",grupo_a[i].equipo2);
    
            io::stdin().read_line(&mut gols2).expect("fallo la lectura");
            
            let mut gols2=gols2.trim().to_string();
    
            if gols2.parse::<u8>().is_ok(){
 
    
            }else {
                continue;
            }
    
            let gols2:u8=gols2.parse().expect("fallo");

            grupo_a[i].gol2=gols2;

            break;

            }

            if grupo_a[i].gol1>grupo_a[i].gol2{

             for j in  0..equipo.len(){

             if equipo[j].nombre==grupo_a[i].equipo1{


                equipo[j].pj+=1;
                equipo[j].pg+=1;
                equipo[j].gf+=&grupo_a[i].gol1;
                equipo[j].ge+=&grupo_a[i].gol2;
                equipo[j].puntos+=3;


             }
              
            if equipo[j].nombre==grupo_a[i].equipo2{

                equipo[j].pj+=1;
                equipo[j].pp+=1;
                equipo[j].gf+=&grupo_a[i].gol2;
                equipo[j].ge+=&grupo_a[i].gol1;
            

            }
             
             }

            }

            if grupo_a[i].gol1<grupo_a[i].gol2{

                for j in  0..equipo.len(){
   
                if equipo[j].nombre==grupo_a[i].equipo1{
   
   
                   equipo[j].pj+=1;
                   equipo[j].pp+=1;
                   equipo[j].gf+=&grupo_a[i].gol1;
                   equipo[j].ge+=&grupo_a[i].gol2;
   
                }
                 
               if equipo[j].nombre==grupo_a[i].equipo2{
   
                   equipo[j].pj+=1;
                   equipo[j].pg=1;
                   equipo[j].gf+=&grupo_a[i].gol2;
                   equipo[j].ge+=&grupo_a[i].gol1;
                   equipo[j].puntos+=3;
               
   
               }
                
                }
   
               }

               if grupo_a[i].gol1==grupo_a[i].gol2{

                for j in  0..equipo.len(){
   
                if equipo[j].nombre==grupo_a[i].equipo1{
   
   
                   equipo[j].pj+=1;
                   equipo[j].pe+=1;
                   equipo[j].gf+=&grupo_a[i].gol1;
                   equipo[j].ge+=&grupo_a[i].gol2;
                   equipo[j].puntos+=1;

                }
                 
               if equipo[j].nombre==grupo_a[i].equipo2{
   
                   equipo[j].pj+=1;
                   equipo[j].pe=1;
                   equipo[j].gf+=&grupo_a[i].gol2;
                   equipo[j].ge+=&grupo_a[i].gol1;
                   equipo[j].puntos+=1;
               
   
               }
                
                }
   
               }

        }
            println!("Grupo B");

            for i in 0..grupo_b.len(){
       
               let mut gols1=String::new();
       
               let  mut gols2=String::new();
       
               println!();
       
               println!("{} vs {}",grupo_b[i].equipo1,grupo_b[i].equipo2);
               
              loop{
       
               println!("Goles del equipo {}",grupo_b[i].equipo1);
       
               io::stdin().read_line(&mut gols1).expect("fallo la lectura");
               
               let mut gols1=gols1.trim().to_string();
       
               if gols1.parse::<u8>().is_ok(){
       
               }else {
                   continue;
               }
       
               let gols1:u8=gols1.parse().expect("fallo");
       
               grupo_b[i].gol1=gols1;
       
               break;
       
               }
       
               loop{
       
                   println!("Goles del equipo {}",grupo_b[i].equipo2);
           
                   io::stdin().read_line(&mut gols2).expect("fallo la lectura");
                   
                   let mut gols2=gols2.trim().to_string();
           
                   if gols2.parse::<u8>().is_ok(){
        
           
                   }else {
                       continue;
                   }
           
                   let gols2:u8=gols2.parse().expect("fallo");
       
                   grupo_b[i].gol2=gols2;
       
                   break;
       
                   }    

                   if grupo_b[i].gol1>grupo_b[i].gol2{

                    for j in  0..equipo.len(){
       
                    if equipo[j].nombre==grupo_b[i].equipo1{
       
       
                       equipo[j].pj+=1;
                       equipo[j].pg+=1;
                       equipo[j].gf+=&grupo_b[i].gol1;
                       equipo[j].ge+=&grupo_b[i].gol2;
                       equipo[j].puntos+=3;
       
       
                    }
                     
                   if equipo[j].nombre==grupo_b[i].equipo2{
       
                       equipo[j].pj+=1;
                       equipo[j].pp+=1;
                       equipo[j].gf+=&grupo_b[i].gol2;
                       equipo[j].ge+=&grupo_b[i].gol1;
                   
       
                   }
                    
                    }
       
                   }
       
                   if grupo_b[i].gol1<grupo_b[i].gol2{
       
                       for j in  0..equipo.len(){
          
                       if equipo[j].nombre==grupo_b[i].equipo1{
          
          
                          equipo[j].pj+=1;
                          equipo[j].pp+=1;
                          equipo[j].gf+=&grupo_b[i].gol1;
                          equipo[j].ge+=&grupo_b[i].gol2;
          
                       }
                        
                      if equipo[j].nombre==grupo_b[i].equipo2{
          
                          equipo[j].pj+=1;
                          equipo[j].pg=1;
                          equipo[j].gf+=&grupo_b[i].gol2;
                          equipo[j].ge+=&grupo_b[i].gol1;
                          equipo[j].puntos+=3;
                      
          
                      }
                       
                       }
          
                      }
       
                      if grupo_b[i].gol1==grupo_b[i].gol2{
       
                       for j in  0..equipo.len(){
          
                       if equipo[j].nombre==grupo_b[i].equipo1{
          
          
                          equipo[j].pj+=1;
                          equipo[j].pe+=1;
                          equipo[j].gf+=&grupo_b[i].gol1;
                          equipo[j].ge+=&grupo_b[i].gol2;
                          equipo[j].puntos+=1;
       
                       }
                        
                      if equipo[j].nombre==grupo_a[i].equipo2{
          
                          equipo[j].pj+=1;
                          equipo[j].pe=1;
                          equipo[j].gf+=&grupo_b[i].gol2;
                          equipo[j].ge+=&grupo_b[i].gol1;
                          equipo[j].puntos+=1;
                      
          
                      }
                       
                       }
          
                      }
        
     }

     let ordenado=self._quicksort(equipo.to_vec());
     
     *equipo=ordenado;
    
   self.opciones(equipo, grupo_a, grupo_b)

   }  


   fn _quicksort(&self,lista:Vec<Equipos>)->Vec<Equipos>{


    if lista.len()<=1{

       return  lista; 
    }
    
    let pivote=lista[0].puntos; 

    let mut menor:Vec<Equipos>=vec![];
    
    let mut igual:Vec<Equipos>=vec![];
    
    let mut mayor:Vec<Equipos>=vec![];
    
    for i in 0..lista.len(){
    
    match lista[i].puntos.cmp(&pivote) {
    
        Ordering::Less=>menor.push(lista[i].clone()),
        Ordering::Equal=>igual.push(lista[i].clone()),
        Ordering::Greater=>mayor.push(lista[i].clone())
        
    };
    
    }
     
    menor=self._quicksort( menor );

    mayor=self._quicksort(  mayor);
    
    mayor.extend(igual);

    mayor.extend(menor);

    mayor

}

fn  buscar_equipo(&self,equipo:& mut Vec<Equipos>,grupo_a:& mut Vec<Partido>,grupo_b:& mut Vec<Partido>){

   print!("{}[2J", 27 as char);
   
   println!("Ingrese el nombre del equipo que desea buscar");

   let mut nombre=String::new();

   io::stdin().read_line(& mut nombre).expect("fallo");

   nombre=nombre.trim().to_string();

    for i in 0..equipo.len(){

         if equipo[i].nombre==nombre{

            println!("El equipo {} esta en grupo {} y en la pocision {}",nombre,equipo[i].grupo,i+1);

            self.opciones(equipo, grupo_a, grupo_b);            

         }
       
       
    }

    println!("No sea encontrado al equipo");

    let mut pausa=String::new();

    io::stdin().read_line(&mut pausa).expect("fallo");

    self.opciones(equipo, grupo_a, grupo_b);

}

fn mostrar_encuentros(&self,equipo:& mut Vec<Equipos>,grupo_a:& mut Vec<Partido>,grupo_b:& mut Vec<Partido>){

    print!("{}[2J", 27 as char);

    println!("Grupo A");

    for i in 0..grupo_a.len(){
    
    println!();

    println!("{} vs {}",grupo_a[i].equipo1,grupo_a[i].equipo2);

    }

    println!("Grupo B");

    for i in 0..grupo_b.len(){
    
        println!();
    
        println!("{} vs {}",grupo_b[i].equipo1,grupo_b[i].equipo2);
    
    
        }
    
    let mut pausa=String::new();

    io::stdin().read_line(&mut pausa).expect("fallo");

    self.opciones(equipo, grupo_a, grupo_b);

}

fn mostrar_resultados(&self,equipo:& mut Vec<Equipos>,grupo_a:& mut Vec<Partido>,grupo_b:& mut Vec<Partido>){

  print!("{}[2J", 27 as char);
  
  println!("Resultados que desea octener indique el grupo");

  let mut grupo=String::new();

  io::stdin().read_line(&mut grupo).expect("fallo");

  grupo=grupo.trim().to_string();


  if grupo=="A" || grupo=="a"{

     for i in 0..grupo_a.len(){

        println!();

        println!("{} vs {}",grupo_a[i].equipo1,grupo_a[i].equipo2);

        println!("{} : {}",grupo_a[i].gol1,grupo_a[i].gol2);

     }

  }else {
      
    for i in 0..grupo_b.len(){

        println!();

        println!("{} vs {}",grupo_b[i].equipo1,grupo_b[i].equipo2);

        println!("{} : {}",grupo_b[i].gol1,grupo_b[i].gol2);

     }

     

  }

  let mut pausa=String::new();

  io::stdin().read_line(&mut pausa).expect("fallo");

  self.opciones(equipo, grupo_a, grupo_b)
}

fn opciones(&self,equipo:&mut Vec<Equipos>,grupo_a:&mut Vec<Partido>,grupo_b:&mut Vec<Partido>){

print!("{}[2J", 27 as char);

println!("1.Cargar equipos");
println!();
println!("2.cargar resultados de los partidos");
println!();
println!("3.mostrar resultados");
println!();
println!("4.tablas del torneo");
println!();
println!("5.buscar equipo");
println!();
println!("6.tablas del torneo");

let mut opcion=String::new();

io::stdin().read_line(&mut opcion).expect("fallo");

opcion=opcion.trim().to_string();

if opcion=="1" {

    self.cargar(equipo, grupo_a, grupo_b);
    
}

    
if opcion=="2" {

    self.ingresar_resultados(equipo, grupo_a, grupo_b);
    
}

if opcion=="3" {

    self.mostrar_resultados(equipo,grupo_a, grupo_b);
    
}

if opcion=="4" {

    self.tabla(equipo,grupo_a, grupo_b);
    
}

}

fn tabla(&self,equipo:&mut Vec<Equipos>,grupo_a:&mut Vec<Partido>,grupo_b:&mut Vec<Partido>){

    print!("{}[2J", 27 as char);

    println!("Tablas de posiciones del torneo");

    println!();

    println!("nombre  PJ  PG  PE  PP  GF  GE  Puntos");

    for i in 0..equipo.len(){
    
        println!();
        equipo[i].mostrar();

    }

  let mut pausa=String::new();

  io::stdin().read_line(&mut pausa).expect("fallo");
  
  self.opciones(equipo, grupo_a, grupo_b);

}

}

fn main() {

   let mut equipo:Vec<Equipos>=vec![];

   let mut grupo_a:Vec<Partido>=vec![];

   let mut grupo_b:Vec<Partido>=vec![];

   let incio=menu{};

   incio.opciones(&mut equipo, &mut grupo_a, &mut grupo_b);


    
}

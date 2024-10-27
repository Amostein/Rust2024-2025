fn main() {
    let s1: String = String::from("strings are fun");
    let s2: String = String::from("🎁🎶🎉👀🎈🎃🍕☕🍉");
    let s3: String = String::from("rust");
    let s4: String = String::from("supercalifragilisticexpialidocious");
    
    let mut leng: [usize; 4] = [0,0,0,0];
    
    leng[0]=s1.chars().count();
    leng[1]=s2.chars().count();
    leng[2]=s3.chars().count();
    leng[3]=s4.chars().count();
    let mut i;
    let mut ok=false;
    let mut aux;
    print!("{} {} {} {}\n", leng[0], leng[1], leng[2], leng[3]);
    while ok == false {
        ok = true;
        i=0;
        while i < 3 {
            if leng[i] < leng[i+1] 
                {
                    ok = false;
                    aux=leng[i];
                    leng[i]=leng[i+1];
                    leng[i+1]=aux;
                }
            i=i+1;
        }
    }
    ok=false;
    if leng[0]==s1.chars().count(){
        print!("{}\n",s1);
    }
    else
    {
        if leng[0]==s2.chars().count(){
            print!("{}\n",s2);
        }
        else
        {
            if leng[0]==s3.chars().count(){
                 print!("{}\n",s3);
             }
            else
            {   if leng[0]==s4.chars().count(){
                print!("{}\n",s4);
                }
             }
        }
    }
    

    let mut lung: [usize; 4] = [0,0,0,0];
    lung[0]=s1.len();
    lung[1]=s2.len();
    lung[2]=s3.len();
    lung[3]=s4.len();
    while ok == false {
        ok = true;
        i=0;
        while i < 3 {
            if lung[i] < lung[i+1] 
                {
                    ok = false;
                    aux=lung[i];
                    lung[i]=lung[i+1];
                    lung[i+1]=aux;
                }
            i=i+1;
        }
    }
    if lung[0]==s1.len(){
        print!("{}\n",s1);
    }
    else
    {
        if lung[0]==s2.len(){
            print!("{}\n",s2);
        }
        else
        {
            if lung[0]==s3.len(){
                 print!("{}\n",s3);
             }
            else
            {   if lung[0]==s4.len(){
                print!("{}\n",s4);
                }
             }
        }
    }
    
}

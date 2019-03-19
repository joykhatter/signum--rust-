

fn str_concat(word1: &String, word2: &String) -> String
{
    format!("{}{}",word1,word2);
}

fn average_values(arr1: &mut [f64], (up,low): (int,int)) -> f64
{    
    let mut avg: f64;
	let avg =0;
    let mut i: i32;
    let i = up - low;	
	for j in (0..i).rev()
	{
		let avg = avg + arr1[low + j];
	}
	let avg = avg / i;
	avg;
}

fn signum_arr(arr2: &mut [i32]) -> arr2: [i32]
{   
    let mut num: i32;
	let num= arr2.len();
    for i in (0..num).rev()
	{
		if arr2[i]<0 {let arr2[i] = -1}
        else if arr2[i]>0 {let arr2[i] = 1}
        else {let arr2[i] =0	};	
	}
	arr2;
}




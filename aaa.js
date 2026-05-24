const array = ["string", "da", "enervez"];
function stringuri(array) {
    const consoane=`qwrtpsdfghjklzxcvbnm`;
    let nr=0;
    let ccc=[];
    let aaa=[];

    array.forEach(element => {
        for (let i=0; i<element.length;i++) {
            if (consoane.includes(element[i])) {
                nr++;
                ccc.push(element[i]);
            }
        }
        ccc.push('-');
    });
    let j=0;
    for (let i=0; i<ccc.length; i++) {
        if (ccc[i]==='-') {
            if (j===3) {break;} 
            else {j++;}}
        else{aaa.push(ccc[i]+array[j].length);}
    }
    return aaa;
}
console.log(stringuri(array));
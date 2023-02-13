import { execSync } from 'child_process';
 
export function getAddressFromName(binary: string, name: string) {
const cmd = binary + ' keys list --output json';
let result = execSync(cmd)
    let jsonData = JSON.parse(result.toString());

    for (let key in jsonData) { 
        if (name == jsonData[key].name.toString()) {
            return jsonData[key].address.toString();
        }
    }
    return ""
}

// get address for alice
console.log(getAddressFromName("wasmd", "alice"))

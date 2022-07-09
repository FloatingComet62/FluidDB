import fs from 'fs'
import path from 'path'

function getData(): { success: boolean, message: any, code: number } {
    try {
        const raw = fs.readFileSync(
            path.join(__dirname, 'data.json'), 
            'utf8'
        )
        try { return { success: true, message: JSON.parse(raw), code: 0 } }
        catch { return { success: false, message: 'Error parsing JSON', code: 1 } }
    } catch { return { success: false, message: 'Error getting data', code: 2 } }
}
function setData(data: Object) {
    try {
        const parsed = JSON.stringify(data)
        try {
            fs.writeFileSync(
                path.join(__dirname, 'data.json'),
                parsed,
                { encoding: 'utf-8' }
            )
            return { success: true, message: 'Data saved', code: 10 }
        } catch { return { success: false, message: 'Error saving data', code: 11 } }
    } catch { return { success: false, message: 'Error parsing data', code: 12 } }
}


export default {
    getData,
    setData
}
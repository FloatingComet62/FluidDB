import fs from 'fs'
import path from 'path'
import { LowResponse } from './types'



function getData(): LowResponse {
    try {
        const raw = fs.readFileSync(
            path.join(__dirname, 'data.json'), 
            'utf8'
        )
        try { return { success: true, message: JSON.parse(raw) } }
        catch { return { success: false, message: 'Error parsing JSON' } }
    } catch { return { success: false, message: 'Error getting data' } }
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
            return { success: true, message: 'Data saved' }
        } catch { return { success: false, message: 'Error saving data' } }
    } catch { return { success: false, message: 'Error parsing data' } }
}


export default {
    getData,
    setData
}
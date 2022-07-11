export type LowResponse = {
    success: true
    message: Database
} | {
    success: false
    message: string
}

export type HighResponse = {
    success: true
    message: Database
    code: number
} | {
    success: false
    message: string
    code: number
}

export type HighGetResponse = {
    success: true
    message: 
    { [key: string]: { [key: string]: string; }; } // ocean
    | { [key: string]: string; } // sea
    | string // river
    code: number
} | {
    success: false
    message: string
    code: number
}

type Database = {
    [key: string]: {
        [key: string]: {
            [key: string]: string
        }
    }
}
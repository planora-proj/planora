export interface SignInFormData {
    email: string;
    password: string;
}

export interface SignInFormActionResponse {
    success: boolean;
    message: string;
    errors?: {
        [K in keyof SignInFormData]?: string[];
    };
    values?: SignInFormData;
    redirectTo?: string;
}

export interface SignUpFormData {
    username: string;
    email: string;
    password: string;
}

export interface SignUpFormActionResponse {
    success: boolean;
    message: string;
    errors?: {
        [K in keyof SignUpFormData]?: string[];
    };
    values?: SignUpFormData;
    redirectTo?: string;
}

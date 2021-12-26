import { hash } from 'bcrypt';
import { Entity, Column, PrimaryGeneratedColumn, CreateDateColumn, BeforeInsert } from 'typeorm';

@Entity()
export class User {
    
    @PrimaryGeneratedColumn()
    id: number;

    @Column()
    role: string;
    
    @Column()
    name: string;

    @Column()
    email: string;

    @Column()
    password: string;

    @Column()
    passwordConfirm: string;

    @Column()
    active: boolean;
    static password: String;
    static id: any;

    @BeforeInsert()
    async roleDefault(): Promise<void> {
        this.role = 'user';
    }

    @BeforeInsert()
    async hashPassword(): Promise<void> {
        this.password = await hash(this.password, 10);
        this.passwordConfirm = this.password;
    }

    
}
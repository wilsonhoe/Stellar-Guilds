import {
  IsString,
  IsOptional,
  MaxLength,
  MinLength,
  IsObject,
  Matches,
} from 'class-validator';
import { Transform } from 'class-transformer';

export class CreateGuildDto {
  @IsString()
  @MaxLength(100)
  @MinLength(1)
  name!: string;

  @IsOptional()
  @IsString()
  @MinLength(1)
  @MaxLength(100)
  @Transform(({ value }) => value?.toLowerCase().trim())
  @Matches(/^[a-z0-9-]+$/, {
    message: 'slug must contain only lowercase letters, numbers, and hyphens',
  })
  slug?: string;

  @IsOptional()
  @IsString()
  @MaxLength(1000)
  description?: string;

  @IsOptional()
  @IsObject()
  settings?: any;
}

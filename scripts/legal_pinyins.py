def generate_legal_pinyins() -> set[str]:
    result = set()
    consonant_list = 'b,p,m,f,d,t,n,l,g,k,h,j,q,x,z,c,s,r,zh,ch,sh,y,w'.split(',')
    rhyme_list = 'a,o,e,i,u,v,ai,ei,ui,ao,ou,iu,ie,ve,er,an,en,in,un,ang,eng,ing,ong,uai,ia,uan,uang,uo,ua'.split(',')
    integral_syllable_list = 'a,o,e,ai,ei,ao,ou,er,an,en,ang,zi,ci,si,zhi,chi,shi,ri,yi,wu,yu,yin,ying,yun,ye,yue,yuan'.split(',')
    extra = "ng,hng".split(',')

    for s in consonant_list:
        for y in rhyme_list:
            result.add(s + y)

    for i in integral_syllable_list:
        result.add(i)

    for i in extra:
        result.add(i)

    return result

if __name__ == '__main__':
    pinyins = generate_legal_pinyins()
    contents = "\n".join(map(lambda py: f"    \"{py}\",", pinyins))
    print(f"pub const LEGAL_PINYINS: [&str; {len(pinyins)}] = [\n{contents}\n];")

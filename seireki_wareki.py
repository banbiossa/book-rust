start_year_dict = {
    '明治': 1868,
    '大正': 1912,
    '昭和': 1926,
    '平成': 1989,
    '令和': 2019,
}

start_years = [(i, v) for i, v in start_year_dict.items()]


def get_era(year) -> tuple[str, int]:
    for era, start_year in start_years[::-1]:
        if year >= start_year:
            return era, start_year


def get_wareki(year):
    era, start_year = get_era(year)
    if year == start_year:
        year = '元'
    else:
        year = year - start_year + 1
    return f"{era}{year}年"


for i in range(1926, 2027):
    print(f"西暦{i}年 = {get_wareki(i)}")

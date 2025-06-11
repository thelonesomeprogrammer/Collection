from re import split
import requests
import json
from surrealdb import Surreal
import os
import asyncio



def rename(filename:str) -> str:
        if "dont-fuck-my" in filename:
                return filename
        if "tushy" in filename:
                return filename
        if "cum4k" in filename:
                return filename.replace("cum4k","cum-4k").replace("-2160","")
        match filename.split("_")[0]:
            case "princesscum":
                return "?site=princess-cum&external_id="+filename.split("_")[1]
            case "bffs":
                    return filename.replace(filename.split("_")[0],"bffs")
            case "sislovesme":
                    return filename.replace(filename.split("_")[0],"sis-loves-me").replace("dungeonsanddick","dungeonsdick")
            case "brattysis":
                    return "?site=bratty-sis&external_id="+filename.split("_")[1]
            case "mybabysittersclub":
                    return filename.replace(filename.split("_")[0],"my-babysitters-club")
            case "familystrokes":
                    return filename.replace(filename.split("_")[0],"family-strokes")
            case "stepsiblingscaught":
                    return "?site=step-siblings-caught&external_id="+filename.split("_")[1]
            case "shesnew":
                    return filename.replace(filename.split("_")[0],"shes-new")
            case "familyswap":
                return "?site=family-swap&external_id="+filename.split("_")[1]
            case "myfamilypies":
                    return "?site=my-family-pies&external_id="+filename.split("_")[1]
            case "thisgirlsucks":
                    return filename.replace(filename.split("_")[0],"this-girl-sucks")
            case "exxxtrasmall":
                    return filename.replace(filename.split("_")[0],"exxxtra-small")
            case "momsteachsex":
                    return "?site=moms-teach-sex&external_id="+filename.split("_")[1]
            case "teacherfucksteens":
                    return "?site=teacher-fucks-teens&external_id="+filename.split("_")[1]
            case "innocenthigh":
                      return filename.replace(filename.split("_")[0],"innocent-high")
            case "freeusefantasy":
                      return filename.replace(filename.split("_")[0],"freeuse-fantasy")
            case "petitehdporn":
                      return "?site=petite-hd-porn&external_id="+filename.split("_")[1]
            case "spyfam":
                      return filename.replace(filename.split("_")[0],"spy-fam").replace("_.mp4","").replace(".mp4","")
            case "dadcrush":
                      return filename.replace(filename.split("_")[0],"dad-crush")
            case "detentiongirls":
                      return "?site=detention-girls&external_id="+filename.split("_")[1]
            case "teensloveanal":
                      return filename.replace(filename.split("_")[0],"teens-love-anal")
            case "pervmom":
                      return filename.replace(filename.split("_")[0],"perv-mom")
            case "povlife":
                      return filename.replace(filename.split("_")[0],"pov-life")
            case "badmilfs":
                      return filename.replace(filename.split("_")[0],"bad-milfs")
            case "stepsiblings":
                        return filename.replace(filename.split("_")[0],"step-siblings")
            case "stayhomepov":
                        return filename.replace(filename.split("_")[0],"stay-home-pov")
            case "daddyslilangel":
                        return "?site=daddys-lil-angel&external_id="+filename.split("_")[1]
            case "teamskeet":
                        return filename.replace(filename.split("_")[0],"teamskeet-features").replace("freeuse","free-use")
            case "teamskeetfeatures":
                        return filename.replace(filename.split("_")[0],"teamskeet-features")
            case "teamskeetxbang":
                        return filename.replace(filename.split("_")[0],"team-skeet-x-bang")
            case _:
                        print(filename)
                        return filename

async def main():
    async with Surreal("ws://localhost:8000/rpc") as db:
        await db.signin({"user": "root", "pass": "root"})
        await db.use("test", "test")
        for (root, _, filenames) in os.walk('/home/marrinus/website-data'):
            if 'thumbnail' in root:
                continue
            for filename in filenames:
                re = requests.request("GET","https://api.metadataapi.net/scenes/"+rename(filename.lower()).replace(".mp4","").replace("_","-").replace("external-id=","external_id="),headers={"Authorization":"Bearer IwV3wjLciHeY09aF7iKo1WzsPuWXcwYwt72f3yVn"})
                if re.status_code == 404:
                        print("https://api.metadataapi.net/scenes/"+rename(filename).replace(".mp4","").replace("_","-"),re.status_code)
                        continue
                try:
                        json_data = re.json()["data"]
                except:
                        print(re)
                        continue

                if type(json_data)== list:
                        if len(json_data)==1:
                                json_data = json_data[0]
                        else:
                                for i in json_data:
                                        print(i['external_id'])
                                        if i['external_id'] == filename.split("_")[1]:
                                                json_data = i
                                                print(i)
                                if type(json_data)== list:
                                        print(filename)
                                        continue
                if "id" not in json_data.keys():
                        print(json_data)
                        continue

                path = root.replace("/home/marrinus/website-data","")+"/"+filename
                movimg = json_data.get("background",{}).get("full","")

                if "default" in movimg:
                    movimg = json_data.get("image","")

                movieentry = {"filename":str(path),
                             "title":str(json_data.get("title","")),
                             "plot":str(json_data.get("description",'')),
                             "date":str(json_data.get("date","")),
                             "url":str(json_data.get("url","")),
                             "img":str(movimg),}
                
                page = requests.get(json_data["image"])
                with open("/home/marrinus/website-data/thumbnail/"+filename.replace(".mp4","")+".jpg", 'wb') as f:
                        f.write(page.content)

                for i in movieentry.keys():
                    if movieentry[i] == None:
                        movieentry[i] = ""


                movie = await db.create("scene",movieentry)
                
                if json_data.get("site") != None:
                    try:
                        network = json_data["site"].get("network",{}).get("name","")
                    except:
                        network = ""

                    studioentry = {"name":str(json_data["site"].get("name",'')),
                               "description":str(json_data["site"].get("description",'')),
                               "network":str(network),
                               "url":str(json_data["site"].get("url","")),
                               "logo":str(json_data["site"].get("logo",''))}
                    if json_data["site"]["logo"] != None:
                        page = requests.get(json_data["site"]["logo"])
                        with open("/home/marrinus/website-data/thumbnail/"+json_data['site']['short_name']+".jpg", 'wb') as f:
                                f.write(page.content)

                    for i in studioentry.keys():
                        if studioentry[i] == None:
                            studioentry[i] = ""


                    studio = await db.create("studio",studioentry)
                    await db.query("RELATE "+movie[0]["id"]+"->made_by->"+studio[0]["id"])


                for i in json_data["performers"]:
                        if i["extra"]["gender"] != "Female":
                            continue
                        modelentry = {"name":i["name"],"bio":i["bio"]}
                        for j in i["extra"].keys():
                                if j == "astrology":
                                        continue
                                if j == "height":
                                        if i["extra"][j] == None:
                                                modelentry["height"] = None
                                                continue
                                        elif "cm" in i["extra"][j]:
                                                modelentry["height"] = int(i["extra"][j].replace("cm","").strip())
                                                continue
                                        elif "ft" in i["extra"][j]:
                                                modelentry["height"] = int(i["extra"][j].replace("ft","").strip())*30.48
                                                continue
                                        elif "'" in i["extra"][j]:
                                                modelentry["height"] = int(i["extra"][j].split("'")[0].strip())*30.48+int(i["extra"][j].split("'")[1].strip().replace('"',''))*2.54
                                                continue
                                        elif "/" in i["extra"][j]:
                                                modelentry["height"] = int(i["extra"][j].split("/")[0])/int(i["extra"][j].split("/")[1])
                                                continue
                                        else:
                                                modelentry["height"] = int(i["extra"][j].strip())
                                                continue
                                if j == "weight":
                                        if i["extra"][j] == None or i["extra"][j].isnumeric() == False:
                                                modelentry["weight"] = None
                                                continue
                                        elif "kg" in i["extra"][j]:
                                                modelentry["weight"] = int(i["extra"][j].replace("kg","").strip())
                                                continue
                                        elif "lbs" in i["extra"][j]:
                                                modelentry["weight"] = int(i["extra"][j].replace("lbs","").strip())*0.4535924
                                                continue
                                        elif "Lbs" in i["extra"][j]:
                                                modelentry["weight"] = int(i["extra"][j].replace("Lbs","").strip())*0.4535924
                                                continue
                                        else:

                                                print(j,i["extra"][j])
                                                continue
                                if j == "cupsize":
                                        if i["extra"][j] == None:
                                                modelentry["cupsize"] = None
                                                continue
                                        band = int(i["extra"][j][:2])
                                        cop = i["extra"][j][2:]
                                        match cop:
                                                case "AA":
                                                        cop = 0
                                                case "A":
                                                        cop = 1
                                                case "B":
                                                        cop = 2
                                                case "C":
                                                        cop = 3
                                                case "D":
                                                        cop = 4
                                                case "DD":
                                                        cop = 5
                                                case "DDD":
                                                        cop = 6
                                                case "E":
                                                        cop = 5
                                                case "F":
                                                        cop = 6
                                                case "G":
                                                        cop = 7
                                                case _:
                                                        print(cop)
                                                        modelentry["cupsize"] = None
                                                        continue
                                        modelentry["cupsize"] = [band*2.54,cop*2.54]
                                        continue
                                modelentry[j]=str(i["extra"][j])
                        page = requests.get(i["image"])
                        with open("/home/marrinus/website-data/thumbnail/"+i["slug"]+".jpg", 'wb') as f:
                                f.write(page.content)

                
                        model = await db.create("model",modelentry)
                        await db.query("RELATE "+movie[0]["id"]+"->featuring->"+model[0]["id"])




if __name__ == "__main__":
    asyncio.run(main())
